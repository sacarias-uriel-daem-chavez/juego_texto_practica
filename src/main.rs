use csv::{ReaderBuilder, StringRecord};
use std::fs;
use std::collections::HashMap;
use std::io;

const PATH_FILE_CSV: &str = "history.csv";

#[derive(Debug, Clone,)]
struct DatosJugabilidad{
    clase_dato: String,
    tag: String,
    texto: String,
    vida: i32,
    opciones: Vec<DatosJugabilidad>,
}

impl DatosJugabilidad {
    fn new_from(row: StringRecord)-> DatosJugabilidad {
            let datos = DatosJugabilidad {
                clase_dato: row.get(0).unwrap().trim().to_string(),
                tag: row.get(1).unwrap().trim().to_string(),
                texto: row.get(2).unwrap().trim().to_string(),
                vida: row.get(3).unwrap().trim().parse().unwrap_or(0),
                opciones: vec![],
        };
        return datos;
    }

    fn new()-> DatosJugabilidad {
        let datos = DatosJugabilidad {
            clase_dato: "".to_string(),
            tag: "".to_string(),
            texto: "".to_string(),
            vida: 0,
            opciones: vec![],
        };
    return datos;
    }

}


fn main() {
    let mut diccionario_situaciones: HashMap<String, DatosJugabilidad> = HashMap::new();
    let content_csv = fs::read_to_string(PATH_FILE_CSV).unwrap();
    let mut reader_builder_csv = ReaderBuilder::new().delimiter(b';').from_reader(content_csv.as_bytes());
    let mut ultimo_dato_corrido: DatosJugabilidad = DatosJugabilidad::new();

    for datos_csv in reader_builder_csv.records(){
        let datos_string_record= datos_csv.unwrap();
        let datos_corriendo = DatosJugabilidad::new_from(datos_string_record);

        if datos_corriendo.clase_dato == "SITUACION" {
                diccionario_situaciones.insert(datos_corriendo.tag.clone(), datos_corriendo.clone());
                ultimo_dato_corrido = datos_corriendo;
        }
        else if datos_corriendo.clase_dato == "OPCION" {
            ultimo_dato_corrido.opciones.push(datos_corriendo);
            diccionario_situaciones.insert(ultimo_dato_corrido.tag.clone(), ultimo_dato_corrido.clone());
        }
    }

    //game lopp
    let mut vida_jugador: i32 = 100;
    let situacion_inicial = diccionario_situaciones["INICIO"].clone();
    let mut situacion_actual = situacion_inicial;
    
    loop{
        print!("\n {}", situacion_actual.texto);
        vida_jugador += situacion_actual.vida;
        print!("\n Tienes {} de vida\n", vida_jugador);

        if vida_jugador <= 0 {
            print!("\n   ----Fin de juego----");
            break;
        }

        let mut vector_opciones: Vec<String> = vec![];
        for datos in situacion_actual.opciones.clone(){
            vector_opciones.push(datos.texto);
        }

        for (index, opcion) in vector_opciones.iter().enumerate() {
            print!(" {} ---> {}\n", index, opcion);
        }

        let mut input_jugador: String = "".to_string();
        io::stdin().read_line(&mut input_jugador).unwrap();
        let eleccion_del_jugador: usize = input_jugador.trim().parse().unwrap_or(99);

        if let Some(opciones) = situacion_actual.opciones.get(eleccion_del_jugador){
        situacion_actual = diccionario_situaciones[&opciones.tag.un].clone();
        }
    }
}
