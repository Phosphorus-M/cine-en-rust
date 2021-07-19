use super::asiento::Asiento;

#[derive(Debug, Clone)]
pub struct Sala {
    pub asientos: Vec<Vec<Asiento>>,
}

impl Sala {
    pub fn new() -> Sala {
        let mut fila_de_asientos: Vec<Vec<Asiento>> = Vec::with_capacity(10);
        for _i in 0..fila_de_asientos.capacity() {
            let mut columnas_de_asientos: Vec<Asiento> = Vec::with_capacity(30);
            for _j in 0..columnas_de_asientos.capacity() {
                columnas_de_asientos.push(Asiento::new());
            }
            fila_de_asientos.push(columnas_de_asientos);
        }
        Sala {
            asientos: fila_de_asientos,
        }
    }
}
