//------------------------------------------//
//            compiler suggestion in Panic Error//
//-------------------------------------------------//

fn main(){
    a();
}

fn a(){
    b();
}

fn b(){
    c(22);
}
fn c(num: i32){
    if num == 22{
        panic!("Don't pass in 22!");
    }
}