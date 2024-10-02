-- This file should undo anything in `up.sql`
DELETE FROM offers WHERE title IN ('Voiture d\'occasion', 'Appartement à louer', 'Téléphone portable');
