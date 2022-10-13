

use std::rc::Rc;
use std::collections::HashMap;

pub trait Expression  {
    
	fn literal(& self) -> String;

    fn evaluate(& self, env: & mut Environment) ->  Option<Rc<dyn Expression>>;

}


// This trait is implemented by all expressions
pub trait Function : Expression {

    fn invoke(&self, env: & mut Environment, args:  [Rc<dyn Expression>] ) -> Option<Rc<dyn Expression>>;
} 

struct Value  {
	expr :   Rc<dyn Expression>,
	read_only : bool,

}
impl Value  {
    pub fn new( expr: Box< dyn Expression>, read_only: bool) -> Value {
        Value{
            expr :   expr.into(),
            read_only: read_only,
        }
    }

    pub fn get(& self) -> Rc<dyn Expression>{
         return Rc::clone(&self.expr)
    }
}

pub struct Environment {
    global_funcs: HashMap<String,  Value>,
    autoregister_globals:bool

}

impl  Environment {
    pub fn new() ->   Self {
         Environment{
            global_funcs: HashMap::new(),
            autoregister_globals: false
        }
    }

    //Null returns a Null expression from the environment
    pub fn null(& self) ->  Option<Rc<dyn Expression>> {
	    return self.get("null".to_string())
    }

    //Set registers a new expression in the environment
    pub fn set(& mut self, name:  String, expr: Box<dyn Expression>) {
        let val = Value::new(expr,false);     
        self.global_funcs.insert( name,  val);
    }


    //Get a registered expression in the environment
    pub fn get(& self, name: String) ->  Option<Rc<dyn Expression>> {

        match self.global_funcs.get(&name){
            Some( ex) => { 
                return Some(ex.get());          
            },
            None => None
        }
    }
}


pub struct  SymbolExpr{
	name	:  	String,
	scope 	: 	Option<Rc<SymbolExpr>>
}

impl Expression for SymbolExpr {

	 fn literal(& self) -> String {
        match &self.scope {
            None => self.name.clone(),
            Some(s) =>  s.as_ref().literal() + "." + &self.name,

        }
	 }

	fn evaluate (& self, env: & mut Environment) ->  Option<Rc<dyn Expression>>{
		let lit = self.literal();
		return env.get(lit)

	}

}
impl SymbolExpr {
    pub fn new(name: String) ->  SymbolExpr {
         SymbolExpr { 
			name: name,
			scope : None
		}
    }
    
	pub fn new_with_scope(name:  String, scope:  Box<SymbolExpr>  ) -> SymbolExpr {
        SymbolExpr { 
			name: name,
			scope: Some(scope.into())
		}
    }
}

fn main() {
    println!("Hello, world!");
    let  inner = SymbolExpr::new("Inner".to_string());
    let  scoped = SymbolExpr::new_with_scope("Outer".to_string(),Box::new(inner));
    let mut env = Environment::new();
    env.set("Test".to_string(), Box::new(scoped));
    let opt = env.get("Test".to_string()).unwrap();
    

    println!("Literal of scoped: {}", opt.literal());

    println!("Count refs of scoped: {}", Rc::strong_count(&opt));
    drop(env);
    println!("Count refs of scoped: {}", Rc::strong_count(&opt));
}
