fn convert_to_pig_latin(palavra: &String) -> String
{
    let vogais = ["a", "e", "i", "o", "u"];
    let (primeira_letra, resto_palavra) = palavra.split_at(1);
    let primeira_letra_vogal = vogais.contains(&primeira_letra);
    if primeira_letra_vogal
    {
        return format!("{}-hay", palavra);
    }
    return format!("{}-{}ay", resto_palavra, primeira_letra);
}

fn main()
{
    let mut palavra: String;

    palavra = String::from("time");
    println!("{} em pig-latin é {}", palavra, convert_to_pig_latin(&palavra));

    palavra = String::from("amor");
    println!("{} em pig-latin é {}", palavra, convert_to_pig_latin(&palavra));

    palavra = String::from("apple");
    println!("{} em pig-latin é {}", palavra, convert_to_pig_latin(&palavra));

    palavra = String::from("twitter");
    println!("{} em pig-latin é {}", palavra, convert_to_pig_latin(&palavra));
}
