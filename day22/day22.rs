use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut player_cards = contents
        .split("\n\n")
        .map(|block| {
            let mut cards = VecDeque::<usize>::new();
            block
                .lines()
                .skip(1)
                .for_each(|line| cards.push_back(line.parse::<usize>().unwrap()));

            cards
        })
        .collect::<Vec<VecDeque<usize>>>();

    println!("players: {:?}", player_cards);

    while player_cards[0].len() != 0 && player_cards[1].len() != 0 {
        let player1 = player_cards[0].pop_front().unwrap();
        let player2 = player_cards[1].pop_front().unwrap();
        if player1 > player2 {
            player_cards[0].push_back(player1);
            player_cards[0].push_back(player2);
        } else {
            player_cards[1].push_back(player2);
            player_cards[1].push_back(player1);
        }
    }
    println!("player1: {:?}", player_cards[0]);
    println!("player2: {:?}", player_cards[1]);
    player_cards
        .iter()
        .map(|deck| {
            let deck_size = deck.len();

            deck.iter()
                .enumerate()
                .fold(0, |sum, (index, card)| sum + card * (deck_size - index))
        })
        .fold(0, |max, result| if result > max { result } else { max })
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let player_cards = contents
        .split("\n\n")
        .map(|block| {
            let mut cards = VecDeque::<usize>::new();
            block
                .lines()
                .skip(1)
                .for_each(|line| cards.push_back(line.parse::<usize>().unwrap()));

            cards
        })
        .collect::<Vec<VecDeque<usize>>>();

    println!("players: {:?}", player_cards);

    fn recursive_combat(decks: &mut (VecDeque<usize>, VecDeque<usize>), depth: usize) -> bool {
        let mut previous_decks_1 = HashSet::<Vec<usize>>::new();
        let mut previous_decks_2 = HashSet::<Vec<usize>>::new();
        let mut player1_won = true;

        while decks.0.len() != 0 && decks.1.len() != 0 {
            let deck_1_vec = decks.0.iter().map(|&e| e).collect::<Vec<usize>>();
            let deck_2_vec = decks.1.iter().map(|&e| e).collect::<Vec<usize>>();

            if previous_decks_1.contains(&deck_1_vec) || previous_decks_2.contains(&deck_2_vec) {
                return true;
            }

            let cards_key = (deck_1_vec, deck_2_vec);
            let drawn_cards = (decks.0.pop_front().unwrap(), decks.1.pop_front().unwrap());
            let deck_sizes = (decks.0.len(), decks.1.len());

            let won_hand = if deck_sizes.0 >= drawn_cards.0 && deck_sizes.1 >= drawn_cards.1 {
                recursive_combat(
                    &mut (
                        decks
                            .0
                            .iter()
                            .take(drawn_cards.0)
                            .map(|e| *e)
                            .collect::<VecDeque<usize>>(),
                        decks
                            .1
                            .iter()
                            .take(drawn_cards.1)
                            .map(|e| *e)
                            .collect::<VecDeque<usize>>(),
                    ),
                    depth + 1,
                )
            } else {
                drawn_cards.0 > drawn_cards.1
            };

            if won_hand {
                decks.0.push_back(drawn_cards.0);
                decks.0.push_back(drawn_cards.1);
            } else {
                decks.1.push_back(drawn_cards.1);
                decks.1.push_back(drawn_cards.0);
            }

            if decks.0.len() == 0 {
                player1_won = false;
            }
            if decks.1.len() == 0 {
                player1_won = true;
            }

            previous_decks_1.insert(cards_key.0);
            previous_decks_2.insert(cards_key.1);
        }

        player1_won
    }

    let mut decks = (player_cards[0].clone(), player_cards[1].clone());
    let player1_won = recursive_combat(&mut decks, 0);

    let winner_deck = if player1_won { decks.0 } else { decks.1 };
    let deck_size = winner_deck.len();
    winner_deck
        .iter()
        .enumerate()
        .fold(0, |sum, (index, card)| sum + card * (deck_size - index))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
