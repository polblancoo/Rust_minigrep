use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
  //************** */
     let result =  if config.case_sensitive{
        search(&config.query, &contents)
     }else{
        search_case_insensitive(&config.query, &contents)
     };


    //println!("Contenido del archivo : \n{}", contents.trim());
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        //Si no hay 3 argumentos salimos .
        if args.len() < 3 {
            return Err("No Hay suficientes argumentos");
        }
        // args[0] ---contiene el pathn del binario
        let query: String = args[1].clone(); //segundo parametro pasado por linea de comandos.
        let filename: String = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        //devuelve un Ok() type envolviendo el resultado una struct `Config`
        Ok(Config { query, filename, case_sensitive })
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }
    results
}
// no es case sensitive la busqueda
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
   
    let query = query.to_lowercase();
   // println!("{:?}", query2);
    
   let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query: &str = "duct";
        let contents: &str = "
    safe, fast, productive.
    Pick three.
    Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUSt:";
        let contents: &str = "\
Rust:
RUSTACEAN
safe, fast, productive.
Trust me.
Pick three.
";
        assert_eq!(
            vec!["Rust:"],
            search_case_insensitive(query, contents)
        );
    }
}
