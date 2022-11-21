use crate::ast::ASTNode;

pub struct GraphPoints{
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub z: Vec<f32>,

    pub axis_segments: usize
}

impl GraphPoints{
    pub fn new() -> Self{
        Self{
            x: vec![],
            y: vec![],
            z: vec![],
            
            axis_segments: 0
        }
    }

    pub fn get_index(&self, i: usize) -> Vec<f32>{
        return vec![self.x[i],self.y[i],self.z[i]];
    }

    pub fn multi_index_to_single(&self, i: usize, j: usize)->usize{
        j*self.axis_segments + i
    }
}

pub fn execute(ast: &Box<ASTNode>, x: f64, y: f64) -> f64{
    match &**ast {
        ASTNode::BinaryExpr {op, lhs, rhs} =>{
            let lhs_v = execute(&lhs, x, y);
            let rhs_v = execute(&rhs, x, y);

            if *op == "*".to_string(){
                return lhs_v * rhs_v;
            }
            else if *op == "**".to_string(){
                return lhs_v.powf(rhs_v);
            }
            else if *op == "+".to_string(){
                return lhs_v + rhs_v;
            }
            else if *op == "/".to_string(){
                return lhs_v/rhs_v;
            }
            else if *op == "-".to_string(){
                return  lhs_v - rhs_v;
            }

            panic!("Bad habid")
        },
        ASTNode::UnaryExpr { op, lhs } => {
            let lhs_v = execute(&lhs, x, y);
            if *op == "+".to_string(){
                return lhs_v
            }
            else if *op == "-".to_string(){
                return -lhs_v;
            }
            
            panic!("Bad habid")
        },
        ASTNode::MethodCall { name , parameter } => {
            let value = execute(&parameter, x,y);

            if name == "log"{
                return value.log((1.0 as f64).exp())
            }
            else if name == "log10"{
                return value.log10()
            }
            else if name == "log2"{
                return value.log2()
            }
            else if name == "sin"{
                return value.sin()
            }
            else if name == "cos"{
                return value.cos()
            }

            panic!("Bad habid")
        },
        ASTNode::Number( value ) => return *value,
        ASTNode::Variable( variable) => {
            if *variable == "x".to_string(){
                return x;
            }
            else{
                return y;
            }
        }
    }
}

pub fn execute_points(ast: &Box<ASTNode>, x_min: f64, x_max: f64, y_min: f64, y_max: f64, segments: usize) -> GraphPoints{
    let mut graph_points = GraphPoints::new();
    graph_points.axis_segments = segments;

    let x_inc = (x_max - x_min)/((segments as f64)-1.0);
    let y_inc = (y_max - y_min)/((segments as f64)-1.0);

    for j in 0..segments{
        for i in 0..segments{
            let x = x_min + (i as f64)*x_inc;
            let y = y_min + (j as f64)*y_inc;

            graph_points.x.push(x as f32);
            graph_points.y.push(y as f32);
            graph_points.z.push(execute(&ast, x, y) as f32);
        }
    }

    return graph_points;
}
