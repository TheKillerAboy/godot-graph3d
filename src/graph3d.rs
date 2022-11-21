use std::ops::Add;

use gdnative::{prelude::*, api::{MeshInstance, ArrayMesh, Mesh}};

use crate::execute::{GraphPoints,execute_points};
use crate::lexer::lexer;
use crate::parse::parse;

pub struct GraphMeshPoints{
    vertices: Vector3Array,
    indices: Int32Array,
    normals: Vector3Array,
}

impl GraphMeshPoints {
    fn _rust_vec_to_godot_vec(rp: &Vec<f32>) -> Vector3{
        return Vector3{
            x: rp[0],
            y: rp[1],
            z: rp[2],
        }
    }

    pub fn from_graph_points(graph_points: &GraphPoints, render_bottom:bool) -> Self{
        let mut vertices = Vector3Array::new();
        let mut indices = Int32Array::new();
        let mut normals = Vector3Array::new();

        let points_len = graph_points.x.len();

        let mut tmp_normals: Vec<Vector3> = vec![Vector3::new(0.0,0.0,0.0); points_len];

        for i in 0..graph_points.axis_segments-1 {
            for j in 0..graph_points.axis_segments-1 {
                let bottom_left = j*graph_points.axis_segments + i;
                let bottom_right = j*graph_points.axis_segments + i + 1;
                let top_left = (j+1)*graph_points.axis_segments + i;
                let top_right = (j+1)*graph_points.axis_segments + i + 1;


                // counter-clockwise - top
                indices.push(top_left as i32);
                indices.push(bottom_left as i32);
                indices.push(top_right as i32);

                indices.push(bottom_left as i32);
                indices.push(bottom_right as i32);
                indices.push(top_right as i32);
                
                // clockwise - bottom
                if render_bottom{
                    indices.push((points_len + top_right) as i32);
                    indices.push((points_len + bottom_right) as i32);
                    indices.push((points_len + bottom_left) as i32);

                    indices.push((points_len + top_right) as i32);
                    indices.push((points_len + bottom_left) as i32);
                    indices.push((points_len + top_left) as i32);
                }
            }
        }

        // top
        for i in 0..graph_points.axis_segments-1 {
            for j in 0..graph_points.axis_segments-1 {
                let ix1 = graph_points.multi_index_to_single(i, j);
                let ix2 = graph_points.multi_index_to_single(i+1, j);
                let ix3 = graph_points.multi_index_to_single(i, j+1);

                let p1 = GraphMeshPoints::_rust_vec_to_godot_vec(&graph_points.get_index(ix1));
                let p2 = GraphMeshPoints::_rust_vec_to_godot_vec(&graph_points.get_index(ix2));
                let p3 = GraphMeshPoints::_rust_vec_to_godot_vec(&graph_points.get_index(ix3));

                let pn = (p2-p3).cross(p1-p2).normalized();

                tmp_normals[ix1] = tmp_normals[ix1].add(pn);
                tmp_normals[ix2] = tmp_normals[ix2].add(pn);
                tmp_normals[ix3] = tmp_normals[ix3].add(pn);
            }
        }

        for i in 0..points_len{
            tmp_normals[i] = tmp_normals[i].normalized();
            normals.push(tmp_normals[i]);
            vertices.push(GraphMeshPoints::_rust_vec_to_godot_vec(&graph_points.get_index(i)));
        }

        // bottom
        if render_bottom{
            for i in 0..graph_points.x.len(){
                let mut vertex = vertices.get(i as i32);
                let mut normal = normals.get(i as i32);

                vertex.z = vertex.z - 0.01;
                normal = normal * -1.0;

                vertices.push(vertex);
                normals.push(normal);
            }
        }

        return GraphMeshPoints{vertices: vertices, indices: indices, normals: normals};
    }

    pub fn to_array_mesh(self) -> Ref<ArrayMesh>{
        let array_mesh = ArrayMesh::new();

        let arrays = VariantArray::new();

        arrays.resize(ArrayMesh::ARRAY_MAX as i32);

        arrays.set(ArrayMesh::ARRAY_VERTEX as i32,self.vertices);
        arrays.set(ArrayMesh::ARRAY_INDEX as i32,self.indices);
        arrays.set(ArrayMesh::ARRAY_NORMAL as i32,self.normals);

        array_mesh.add_surface_from_arrays(Mesh::PRIMITIVE_TRIANGLES, arrays.into_shared(), VariantArray::new_shared(), 2194432);

        return array_mesh.into_shared();
    }
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Graph3D;

#[methods]
impl Graph3D {
    fn new(_owner: &Node) -> Self {
        Graph3D
    }

    #[method]
    fn modify_mesh(&self, mesh: Ref<MeshInstance, Shared>, formula: String, x_min: f64, x_max: f64, y_min: f64, y_max: f64, segments: usize, render_bottom: bool){
        let mut lexer_stack = lexer(&formula).unwrap();
        let ast = parse(&mut lexer_stack).unwrap();
        let graph_points = execute_points(&ast, x_min, x_max, y_min, y_max, segments);

        let graph_mesh_points = GraphMeshPoints::from_graph_points(&graph_points, render_bottom);

        unsafe { mesh.assume_safe().set_mesh(graph_mesh_points.to_array_mesh()) };
    }
}
