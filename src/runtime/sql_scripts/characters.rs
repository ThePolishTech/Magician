use std::collections::HashMap;

use sqlx::{
    sqlite::{
        SqlitePool
    }, Error
};

#[derive(Clone)]
pub enum CharacterClass {
    Martial,
    HalfCaster,
    Caster
}
impl Into<u8> for CharacterClass {
    fn into(self) -> u8 {
        match self {
            Self::Martial    => 1,
            Self::HalfCaster => 2,
            Self::Caster     => 3
        }
    }
}
#[derive(Clone)]
pub struct Character {
    pub name: String,
    species: String,
    alignment: String,
    likes: String,
    dislike: String,
    motivations: String,
    companions: String,
    backstory: String,
    appearance: String,
    extras: String,
    class: CharacterClass
}
impl Character {
    pub fn from_hashmap_cache(data_in: &HashMap<String, String>) -> Result<Character, String> {

        let fields = [
            "name",
            "species",
            "alignment",
            "likes",
            "dislikes",
            "motivations",
            "companions",
            "backstory",
            "appearance",
            "extra",
            "class"
        ];

        let mut missing_fields = vec![];
        for field in fields {
            if !data_in.contains_key(field) {
                missing_fields.push(field);
            }
        }
        if !missing_fields.is_empty() {
            let mut error_message = String::from(
                "Missing field(s): `"
            );
            for field in missing_fields {
                error_message.push_str(&format!( "{field}, " ));
            }
            error_message.push('`');

            return Err(error_message);
        }

        Ok(Character {
            name: data_in["name"].clone(),
            species: data_in["species"].clone(),
            alignment: data_in["alignment"].clone(),
            likes: data_in["likes"].clone(),
            dislike: data_in["dislikes"].clone(),
            motivations: data_in["motivations"].clone(),
            companions: data_in["companions"].clone(),
            backstory: data_in["backstory"].clone(),
            appearance: data_in["appearance"].clone(),
            extras: data_in["extra"].clone(),
            class: match data_in["class"].as_str() {
                "martial" => CharacterClass::Martial,
                "half-caster" => CharacterClass::HalfCaster,
                "caster" => CharacterClass::Caster,
                other_class => return Err(String::from( format!("Invalid Class Recived: `{other_class}`") ))
            }
        })
    }
}

pub async fn insert_character(database_conn_pool: &SqlitePool, user_id: u64, character_in: Character) -> Result<(), Error> {
    let mut transaction = database_conn_pool.begin().await?;


    sqlx::query(
        "INSERT INTO Characters VALUES ( \
            (
                SELECT IFNULL(MAX(pk_characterID), 0) + 1 FROM Characters
            ), \
            $1, \
            $2, $3, $4, $5, $6, $7, $8, $9, $10, $11 \
        );"
    )
        .bind(user_id as i64)
        .bind(character_in.name)
        .bind(character_in.species)
        .bind(character_in.alignment)
        .bind(character_in.likes)
        .bind(character_in.dislike)
        .bind(character_in.motivations)
        .bind(character_in.companions)
        .bind(character_in.backstory)
        .bind(character_in.appearance)
        .bind(character_in.extras)
        .execute(&mut *transaction)
        .await?;


    let character_class_id: u8 = character_in.class.into();

    // Because this is ran in a transaction, we know that the character with the largest ID, must
    // be the one we just inserted
    sqlx::query(
        "INSERT INTO SelectedCharacterClasses VALUES ( \
            (SELECT MAX(pk_characterID) FROM Characters),
            $1
        );"
    )
        .bind(character_class_id)
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await
}
