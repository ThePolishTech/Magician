CREATE TABLE  IF NOT EXISTS    DiscordUsers
(
    pk_discordUserId       INTEGER  PRIMARY KEY,
    fk_currentCharacter    INTEGER -- Can be NOT NULL due to first registration edge case

    -- FOREIGN KEY (fk_currentCharacter)
    -- REFERENCES Characters (pk_characterId)
);

