pub fn tally(match_results: &str) -> String {
    // todo!(
    //     "Given the result of the played matches '{match_results}' return a properly formatted tally table string."
    // );

    use std::collections::HashMap;

    #[derive(Debug, Default)]
    struct MatchRecord {
        name: String,
        plays: u32,
        wins: u32,
        ties: u32,
        lost: u32,
        points: u32,
    }

    let mut match_records = HashMap::new();

    for match_result in match_results.lines() {
        let mut match_result = match_result.split(";");

        let team1 = match_result.next().unwrap().to_string();
        let team1 = match_records
            .entry(team1.clone())
            .or_insert_with(|| MatchRecord {
                name: team1,
                ..Default::default()
            });
        team1.plays += 1;

        let team2 = match_result.next().unwrap().to_string();

        let result = match_result.next().unwrap();
        assert!(match_result.next().is_none());

        match result {
            "win" => {
                team1.wins += 1;
                team1.points += 3;
            }
            "loss" => {
                team1.lost += 1;
            }
            "draw" => {
                team1.ties += 1;
                team1.points += 1;
            }
            _ => unreachable!("{result:?}"),
        }

        let team2 = match_records
            .entry(team2.clone())
            .or_insert_with(|| MatchRecord {
                name: team2,
                ..Default::default()
            });
        team2.plays += 1;

        match result {
            "win" => {
                team2.lost += 1;
            }
            "loss" => {
                team2.wins += 1;
                team2.points += 3;
            }
            "draw" => {
                team2.ties += 1;
                team2.points += 1;
            }
            _ => unreachable!("{result:?}"),
        }
    }

    let mut match_records = match_records.into_values().collect::<Vec<_>>();
    match_records.sort_by(|r1, r2| {
        use std::cmp::Reverse;
        (Reverse(r1.points), &r1.name).cmp(&(Reverse(r2.points), &r2.name))
    });

    let mut table = Vec::with_capacity(match_records.len() + 1);
    table.push("Team                           | MP |  W |  D |  L |  P".to_string());

    match_records.iter().for_each(|r| {
        table.push(format!(
            "{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
            r.name, r.plays, r.wins, r.ties, r.lost, r.points
        ));
    });
    table.join("\n")
}
