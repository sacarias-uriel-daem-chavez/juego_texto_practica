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
    vida: i32

}

impl DatosJugabilidad {
    fn nuevo_diciconario(path_file: &str)-> HashMap<String, DatosJugabilidad> {
        let content_csv = fs::read_to_string(path_file).unwrap();
        let mut reader_builder_csv = ReaderBuilder::new().delimiter(b';').from_reader(content_csv.as_bytes());
        let mut diccionario_output: HashMap<String, DatosJugabilidad> = HashMap::new();
    
        for row_datos_del_juego in reader_builder_csv.records(){
            let row = row_datos_del_juego.unwrap();
            let datos = DatosJugabilidad {
                clase_dato: row.get(0).unwrap().trim().to_string(),
                tag: row.get(1).unwrap().trim().to_string(),
                texto: row.get(2).unwrap().trim().to_string(),
                vida: row.get(3).unwrap().trim().parse().unwrap_or(0)
            };
    
            diccionario_output.insert(datos.tag.clone(), datos.clone());
        }
    
        return diccionario_output;
    }
}
fn main() {
    let mut diccionario_datos_jugabilidad: HashMap<String, DatosJugabilidad> = HashMap::new();
    diccionario_datos_jugabilidad = DatosJugabilidad::nuevo_diciconario(PATH_FILE_CSV);
    //debug_dic(PATH_FILE_CSV);
    print!("\n{:#?}", diccionario_datos_jugabilidad);
    debug(diccionario_datos_jugabilidad);
}



fn debug_dic(path_file: &str){
    let content_csv = fs::read_to_string(path_file).unwrap();
    let mut reader_builder_csv = ReaderBuilder::new().delimiter(b';').from_reader(content_csv.as_bytes());
    let mut diccionario_output: HashMap<String, DatosJugabilidad> = HashMap::new();

    for row_datos_del_juego in reader_builder_csv.records(){
        let row = row_datos_del_juego.unwrap();
        let datos = DatosJugabilidad {
            clase_dato: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            texto: row.get(2).unwrap().trim().to_string(),
            vida: row.get(3).unwrap().trim().parse().unwrap_or(0)
        };

        //print!("Antes de diccionario >>\n {:?}", datos);
        diccionario_output.insert(datos.tag.clone(), datos.clone());
        print!("\n En dicionario >> {:?}", diccionario_output[datos.tag.as_str()]);
    }

    print!("........................................................................................");
    for obj in diccionario_output{
        print!("\nciclo for objetos   {:?}", obj);
    }
}

fn debug(diccionario: HashMap<String, DatosJugabilidad>){
    loop {
        let mut input : String = "".to_string();
        print!("\n\n Ingresa tags, <s> y <enter>para salir --> \n ");
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        print!(">{}<", input);

        if input == "s" || input == "S"{
            break;
        } 
        if input != "" && input != "s" && input != "S"{
            let tag_and_data: DatosJugabilidad = diccionario[&input].clone();
            print!(" >>> {:?}", tag_and_data)
        }
    }
}