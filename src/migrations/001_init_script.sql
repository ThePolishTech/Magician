-- CREATE TABLES --
CREATE TABLE  IF NOT EXISTS  DiscordUsers
(
    pk_discordID          INTEGER  PRIMARY KEY,
    fk_selectedCharacter  INTEGER,

    FOREIGN KEY (fk_selectedCharacter)
    REFERENCES Characters (pk_characterID)
);


CREATE TABLE  IF NOT EXISTS  Characters
(
    pk_characterID  INTEGER  PRIMARY KEY,

    name         TEXT  NOT NULL,
    species      TEXT  NOT NULL,
    alignment    TEXT  NOT NULL,
    likes        TEXT  NOT NULL,
    dislike      TEXT  NOT NULL,
    motivations  TEXT  NOT NULL,
    companions   TEXT  NOT NULL,
    backstory    TEXT  NOT NULL,
    appearance   TEXT  NOT NULL,
    extras       TEXT  NOT NULL
);


CREATE TABLE  IF NOT EXISTS  CharacterStats
(
    pk_fk_characterID  INTEGER  PRIMARY KEY,

    strength           INTEGER  NOT NULL,
    dexterity          INTEGER  NOT NULL,
    perception         INTEGER  NOT NULL,
    knowledge          INTEGER  NOT NULL,
    constitution       INTEGER  NOT NULL,
    casting            INTEGER  NOT NULL,

    FOREIGN KEY (pk_fk_characterID)
    REFERENCES Characters (pk_characterID)
);

CREATE TABLE  IF NOT EXISTS  LoresOfMagic
(
    pk_loreID  INTEGER  PRIMARY KEY,
    loreName   TEXT     NOT NULL
);


CREATE TABLE  IF NOT EXISTS  CharacterUsedLores
(
    pk_fk_loreID       INTEGER  NOT NULL,
    pk_fk_characterID  INTEGER  NOT NULL,
    spentPoints        INTEGER  NOT NULL,

    PRIMARY KEY (pk_fk_loreID, pk_fk_characterID),

    FOREIGN KEY (pk_fk_loreID)      REFERENCES LoresOfMagic(pk_loreID),
    FOREIGN KEY (pk_fk_characterID) REFERENCES Characters(pk_characterID)
);


CREATE TABLE  IF NOT EXISTS  CharacterClasses
(
    pk_classID  INTEGER  PRIMARY KEY,
    className   TEXT     NOT NULL
);


CREATE TABLE  IF NOT EXISTS  SelectedCharacterClasses
(
    pk_fk_characterID  INTEGER  NOT NULL,
    pk_fk_classID      INTEGER  NOT NULL,

    PRIMARY KEY (pk_fk_characterID, pk_fk_classID),

    FOREIGN KEY(pk_fk_characterID) REFERENCES Characters(pk_characterID),
    FOREIGN KEY(pk_fk_classID) REFERENCES CharacterClasses(pk_classID)
);




-- INSERT VALUES --
INSERT INTO LoresOfMagic
  SELECT 1, 'The lore of Flame'
  WHERE NOT EXISTS (
      SELECT 1
      FROM LoresOfMagic
      WHERE pk_loreID = 1
  );

INSERT INTO LoresOfMagic
  SELECT 2, 'The Lore of the Storm'
  WHERE NOT EXISTS (
      SELECT 1
      FROM LoresOfMagic
      WHERE pk_loreID = 2
  );

INSERT INTO LoresOfMagic
  SELECT 3, 'The Lore of Earth'
  WHERE NOT EXISTS (
      SELECT 1
      FROM LoresOfMagic
      WHERE pk_loreID = 3
  );

INSERT INTO LoresOfMagic
  SELECT 4, 'The Lore of Nature'
  WHERE NOT EXISTS (
      SELECT 1
      FROM LoresOfMagic
      WHERE pk_loreID = 4
  );

INSERT INTO LoresOfMagic
  SELECT 5, 'The Lore of Necromancy'
  WHERE NOT EXISTS (
      SELECT 1
      FROM LoresOfMagic
      WHERE pk_loreID = 5
  );

INSERT INTO LoresOfMagic
  SELECT 6, 'The Lore of the Oath'
  WHERE NOT EXISTS (
      SELECT 1
      FROM LoresOfMagic
      WHERE pk_loreID = 6
  );

INSERT INTO LoresOfMagic
  SELECT 7, 'The Draconic Lore'
  WHERE NOT EXISTS (
      SELECT 1
      FROM LoresOfMagic
      WHERE pk_loreID = 7
  );

INSERT INTO LoresOfMagic
  SELECT 8, 'The Lore of Blood'
  WHERE NOT EXISTS (
      SELECT 1
      FROM LoresOfMagic
      WHERE pk_loreID = 8
  );

INSERT INTO LoresOfMagic
  SELECT 9, 'The Lore of Light'
  WHERE NOT EXISTS (
      SELECT 1
      FROM LoresOfMagic
      WHERE pk_loreID = 9
  );

INSERT INTO LoresOfMagic
  SELECT 10, 'The Lore of Darkness'
  WHERE NOT EXISTS (
      SELECT 1
      FROM LoresOfMagic
      WHERE pk_loreID = 10
  );

INSERT INTO LoresOfMagic
  SELECT 11, 'The Lore of Music'
  WHERE NOT EXISTS (
      SELECT 1
      FROM LoresOfMagic
      WHERE pk_loreID = 11
  );


INSERT INTO CharacterClasses
  SELECT 1, 'Martial'
  WHERE NOT EXISTS (
      SELECT 1
      FROM CharacterClasses
      WHERE pk_classID = 1
  );

INSERT INTO CharacterClasses
  SELECT 2, 'Half-Caster'
  WHERE NOT EXISTS (
      SELECT 1
      FROM CharacterClasses
      WHERE pk_classID = 2
  );

INSERT INTO CharacterClasses
  SELECT 3, 'Caster'
  WHERE NOT EXISTS (
      SELECT 1
      FROM CharacterClasses
      WHERE pk_classID = 3
  );


