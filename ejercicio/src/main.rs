use std::io::stdin;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;


#[derive(Default)]
struct Medicamento{
    codigo: String,
    nombre: String,
    componente: String,
    precio: String,
    lab: String
}


fn read_file(mut file: &File) -> String {
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    return text
}
// Lee el archivo y lo devuelve en String

fn create_blank_file(path: &Path){
    let _file = File::create(path).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");

}
// Crea el archivo

fn is_entero_positivo(numero: &str) -> bool {
    for digit in numero.to_string().trim().chars(){
        if digit.is_numeric(){
            continue
        } else {
            return false
        }
    }
    return true
}
// Revisa si es un numero entero positivo

fn open_file_to_append(path: &Path) -> File{
    open_file(path);
    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(path){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };
    return file
}
// Abre el archivo para agregar texto y devuelve archivo

fn open_file_to_write(path: &Path) -> File{
    open_file(path);
    let mut binding = OpenOptions::new();
    let binding = binding.write(true);
    let file = match binding.open(path){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };
    return file
}


fn open_file(path: &Path) -> String{
    let mut text = "".to_string();
    if Path::new(path).exists(){
        let file = match File::open(&path){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        text = read_file(&file);
    } else {
        create_blank_file(path);
    }
    return text
}
// Revisa si existe el archivo

fn menu() -> u32 {
    let mut entrada: String = String::new();
    loop {
        println!("Elija opción:");
        println!("    (1) Agregar un medicamento nuevo.");
        println!("    (2) Consular precio por código del medicamento.");
        println!("    (3) Listar medicamentos por laboratorio.");
        println!("    (4) Listar medicamentos por nombre.");
        println!("    (5) Editar medicamento por código.");
        println!("    (6) Eliminar medicamento por código.");
        println!("    (0) Salir.");
        stdin().read_line(&mut entrada).unwrap();
        //
        if !is_entero_positivo(&entrada) || entrada.trim() == "".to_string() {
            entrada = "".to_string();
            continue
        }
        //
        match entrada.trim().parse().unwrap() {
            0|1|2|3|4|5|6 => break,
            _ => entrada = "".to_string()
        }
        println!("\nIntentelo denuevo\n");
        continue
    }   
    let num: u32 = entrada.trim().parse().unwrap();
    return num
}
// Se elije la función a realizar

fn pedir_medicamento() -> String {
    let mut linea = "".to_string();
    //
    for i in 0..5 {
        loop {
            let mut temp: String = String::new();
            match i {
                0 => println!("Escriba el CÓDIGO del producto") ,
                1 => println!("Escriba el NOMBRE del producto"),
                2 => println!("Escriba el COMPONENTE PRINCIPAL del producto"),
                3 => println!("Escriba el PRECIO del producto"),
                4 => println!("Escriba el LABORATORIO del producto"),
                _ => continue
            };
            stdin().read_line(&mut temp).unwrap();
            //
            if temp.trim() == "".to_string() {
                continue
            }
            //
            if i == 3 {
                if is_entero_positivo(&temp) {
                    linea = linea + &format!("{}", &temp.trim());
                    break
                } else {
                    println!("\nPrecio no válido\n");
                    continue
                }
            }
            //
            linea = linea + &format!("{}", &temp.trim().to_uppercase());
            break
        }
        if i != 4 {
            linea = linea + ":";
        }
    }
    return linea
}
// Pide el medicamento

fn revisar(text: &str, linea: &str) -> bool {
    for lineas in text.split("\n") {
        for dato in lineas.split(":") {
            for a in linea.split(":") {
                if dato == a {
                    if linea.trim() == lineas.trim(){
                        break
                    } else {
                        return false
                    }
                } else {
                    break
                }
            }
            break
        }
    }
    return true
}
// Revisa que el medicamento tenga el mismo codigo

fn crear_structure_med(linea: &str) -> Medicamento {
    let mut med: Medicamento = Default::default();
    let mut contador = 0;
    //
    for b in linea.split(":") {
        match contador {
            0 => med.codigo = b.to_string(),
            1 => med.nombre = b.to_string(),
            2 => med.componente = b.to_string(),
            3 => med.precio = b.to_string(),
            4 => med.lab = b.to_string(),
            _ => continue 
        }
        contador += 1;
    }
    return med
}
// Transforma el dato del archivo a estrcuctura

fn imprimir_medicamento(med: Medicamento) {
    println!("Código: {}", med.codigo);
    println!("Nombre: {}", med.nombre);
    println!("Componente: {}", med.componente);
    println!("Precio: {}", med.precio);
    println!("Laboratorio: {}\n", med.lab)

}
// Muestra el medicamento

fn agregar_medicamento(path: &Path) {
    loop {
        let linea: String = pedir_medicamento().to_string();
        let text: String = open_file(path);
        if revisar(&text, &linea) {
            let mut file: File = open_file_to_append(path);
            file.write_all(linea.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
            break
        }
        println!("\nMedicamento no válido\n")
    }    
}
// Agrega medicamento al archivo

fn consultar_precio(path: &Path) {
    let mut codigo: String = String::new();
    let text:String = open_file(path);
    let mut existen: bool = false;
    //
    println!("Escriba el CODIGO del medicamento:");
    stdin().read_line(&mut codigo).unwrap();
    println!("");
    //
    for linea in text.split("\n") {
        let med = crear_structure_med(linea);
        //
        if med.codigo == codigo.trim().to_uppercase() {
            existen = true;
            imprimir_medicamento(med);
            break
        }
    }
    if !existen {
        println!("Medicamento no encontrado\n")
    }
}
// Consulta el precio del producto

fn listar_laboratorio(path: &Path) {
    let mut laboratorio: String = String::new();
    let text:String = open_file(path);
    let mut existen: bool = false;
    //
    println!("Escriba el LABORAT0RIO del medicamento:");
    stdin().read_line(&mut laboratorio).unwrap();
    println!("");
    //
    for linea in text.split("\n") {
        let med = crear_structure_med(linea);
        if med.lab == laboratorio.trim().to_uppercase() {
            existen = true;
            imprimir_medicamento(med);
        }
    }
    //
    if !existen {
        println!("Medicamento no encontrado\n")
    }
}
// Busca medicamento por laboratorio

fn listar_nombre(path: &Path) {
    let mut nombre: String = String::new();
    let text:String = open_file(path);
    let mut existen: bool = false;
    //
    println!("Escriba el NOMBRE del medicamento:");
    stdin().read_line(&mut nombre).unwrap();
    println!("");
    //
    for linea in text.split("\n") {
        let med = crear_structure_med(linea);
        if med.nombre == nombre.trim().to_uppercase() {
            existen = true;
            imprimir_medicamento(med);
        }
    }
    //
    if !existen {
        println!("Medicamento no encontrado\n")
    }
}
// Busca medicamentos por laboratorio

fn editar_medicamento(path: &Path) {
    let mut codigo: String = String::new();
    let text:String = open_file(path);
    let mut cambiado: bool = false;
    let mut file: File = open_file_to_write(path);
    let mut cadena: String = String::new();
    //
    println!("Escriba el CÓDIGO del medicamento:");
    stdin().read_line(&mut codigo).unwrap();
    println!("");
    //
    for linea in text.split("\n") {
        let med = crear_structure_med(linea);
        if med.codigo == codigo.trim().to_uppercase() && !cambiado{
            println!("El siguente medicamento fue cambiado");
            imprimir_medicamento(med);
            cambiado = true;
            // todo: Se supone que aqui hay que hacer 2 cosas.
            //? 1.- hacer un menu para que elija y ponga que va a cambiar
            //? 2.- funcino para cambiar o un if o match
        }
        if linea.trim() != "" {
            cadena = cadena  + linea + "                  \n";
        }
    }
    //
    //print!("{}", cadena);
    if !cambiado {
        println!("Medicamento no encontrado\n")
    } else {
        //println!("{}", cadena);
        file.write_all(cadena.as_bytes()).unwrap();
    }
}

fn eliminar_medicamento(path: &Path) {
    let mut codigo: String = String::new();
    let text:String = open_file(path);
    let mut eliminado: bool = false;
    let mut file: File = open_file_to_write(path);
    let mut cadena: String = String::new();
    //
    println!("Escriba el CÓDIGO del medicamento:");
    stdin().read_line(&mut codigo).unwrap();
    println!("");
    //
    for linea in text.split("\n") {
        let med = crear_structure_med(linea);
        if med.codigo == codigo.trim().to_uppercase() && !eliminado{
            println!("El siguente medicamento fue eliminado");
            imprimir_medicamento(med);
            eliminado = true;
            continue; 
        }
        if linea.trim() != "" {
            cadena = cadena  + linea + "                  \n";
        }
    }
    //
    //print!("{}", cadena);
    if !eliminado {
        println!("Medicamento no encontrado\n")
    } else {
        //println!("{}", cadena);
        file.write_all(cadena.as_bytes()).unwrap();
    }
}


fn main() {
    let path: &Path = Path::new("base_de_datos.txt");
    //
    loop {
        let opcion = menu();
        match opcion {
            1 => agregar_medicamento(path),
            2 => consultar_precio(path),
            3 => listar_laboratorio(path),
            4 => listar_nombre(path),
            5 => editar_medicamento(path),
            6 => eliminar_medicamento(path),
            _ => break
        }
    }
}
