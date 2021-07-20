use std::io;
use std::str::FromStr;

use super::Asiento;
use super::Sala;

use crate::utils::read_line_and_parse;

#[derive(Debug, Clone)]
pub struct Cine {
    pub nombre: String,
    pub salas: Vec<Sala>,
}

impl FromStr for Cine {
    type Err = ();
    fn from_str(nombre: &str) -> Result<Self, Self::Err> {
        Ok(Cine::new(nombre.trim()))
    }
}

impl Cine {
    pub fn new(nombre: &str) -> Cine {
        let mut salas: Vec<Sala> = Vec::with_capacity(5);
        for _i in 0..salas.capacity() {
            salas.push(Sala::new());
        }

        Cine {
            nombre: nombre.to_string(),
            salas: salas,
        }
    }

    pub fn to_string(&self, sala: usize) -> String {
        let mut s = String::new();
        for i in 0..self.salas[sala].asientos.capacity() {
            for j in 0..self.salas[sala].asientos[i].capacity() {
                if self.salas[sala].asientos[i][j].ocupado {
                    s.push_str("1");
                } else {
                    s.push_str("0");
                }
            }
        }

        s
    }

    // Función para emitir un informe de la sala
    pub fn reporte_sala(&self, sala: usize) -> String {
        let mut s = String::new();
        s.push_str("Sala");
        s.push_str("\n");
        let mut cantidad_de_ocupados = 0;
        for i in 0..self.salas[sala].asientos.capacity() {
            for j in 0..self.salas[sala].asientos[i].capacity() {
                if self.salas[sala].asientos[i][j].ocupado {
                    cantidad_de_ocupados = cantidad_de_ocupados + 1;
                } else {
                    cantidad_de_ocupados = cantidad_de_ocupados + 0;
                }
            }
        }
        s.push_str("Cantidad de asientos ocupados: ");
        s.push_str(cantidad_de_ocupados.to_string().as_str());
        s
    }

    pub fn comprar_entrada(&mut self) {
        let mut sala: usize = 0;
        while sala <= 0 || sala > 6 {
            println!("Seleccione la sala a elegir:");
            sala = read_line_and_parse().unwrap();
            if sala <= 0 || sala > 6 {
                println!("Sala inválida");
                println!("Solo números entre 1 y 5");
            }
        }

        let mut cantidad: u16 = 0;
        while cantidad <= 0 || cantidad > 501 {
            println!("Ingrese la cantidad de entradas a comprar:");
            cantidad = read_line_and_parse().unwrap();
            if cantidad <= 0 || cantidad > 501 {
                println!("Cantidad inválida");
                println!("Solo números entre 1 y 501");
            }
        }

        for _ in 0..cantidad {
            let mut fila: usize = 0;
            while fila <= 0 || fila > 11 {
                println!("Seleccione la fila del asiento a ocupar");
                fila = read_line_and_parse().unwrap();
                if fila <= 0 || fila > 11 {
                    println!("Fila inválida");
                    println!("Solo números entre 1 y 10");
                }
            }

            let mut columna: usize = 0;
            while columna <= 0 || columna > 51 {
                println!("Seleccione la columna del asiento a ocupar");
                columna = read_line_and_parse().unwrap();
                if columna <= 0 || columna > 51 {
                    println!("Fila inválida");
                    println!("Solo números entre 1 y 51");
                }
            }

            self.salas[sala - 1].asientos[fila - 1][columna - 1].ocupado = true;
        }
    }
}
