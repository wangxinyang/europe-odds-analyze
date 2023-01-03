-- Add up migration script here
ALTER TABLE euro.matches ADD predict_game_result VARCHAR(5) DEFAULT NULL;
