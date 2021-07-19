mod lib;
use lib::Cine;
mod utils;
use utils::read_line_and_parse;


fn main() {
    let mut cine = Cine::new("Hoyts");
 
    let mut numero: u8 =0;
    
    while numero < 3{
        println!("Bienvenidos a {}", cine.nombre);
        println!("1 - Comprar entrada");
        println!("2 - Emitir Informe");
        println!("3 - Salir"); 
        
        numero = read_line_and_parse().unwrap();
        
        match numero {
            1 => {
                cine.comprar_entrada();
                cine.to_string( 1);
            },
            2 => {

                println!("{}", cine.reporte_sala(0));
                // println!("{}", cine.reporte_sala(1));
                // println!("{}", cine.reporte_sala(2));
                // println!("{}", cine.reporte_sala(3));
                // println!("{}", cine.reporte_sala(4));

            }
            _ => println!("no c")
        }

    }

}