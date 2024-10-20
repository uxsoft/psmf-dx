use super::models::*;
use scraper::{Html, Selector};
use std::option::Option;

pub async fn search_teams(search_term: String) -> Option<Vec<Team>> {
    use url::Url;

    let url = Url::parse_with_params(
        "https://www.psmf.cz/vyhledavani/",
        &[("ajax", "1"), ("query", &search_term)],
    )
    .ok()?;

    let response = reqwest::get(url).await;
    let html_content = response.ok()?.text().await.ok()?;
    let result: SearchResponse = serde_json::from_str(&html_content).ok()?;

    return Some(result.items);
}

pub async fn get_fields() -> Option<Vec<Field>> {
    let url = "https://www.psmf.cz/hriste/";
    let response = reqwest::get(url).await;
    let html_content = response.ok()?.text().await.ok()?;

    let document = Html::parse_document(&html_content);

    let row_selector = Selector::parse(".cms-editor-table tr").expect("A valid selector");
    let rows = document.select(&row_selector);

    let name_selector = Selector::parse("td:nth-child(1)").expect("A valid selector");
    let key_selector = Selector::parse("td:nth-child(2)").expect("A valid selector");
    let address_selector = Selector::parse("td:nth-child(3)").expect("A valid selector");

    let fields = rows
        .skip(1)
        .flat_map(|row| {
            let name = row
                .select(&name_selector)
                .next()?
                .text()
                .collect::<String>()
                .trim()
                .to_owned();
            let key = row
                .select(&key_selector)
                .next()?
                .text()
                .collect::<String>()
                .trim()
                .to_owned();
            let mut addr_and_notes = row.select(&address_selector).next()?.text();
            let address = addr_and_notes.next()?.to_owned();
            let notes = addr_and_notes.collect::<String>();

            Some(Field {
                key,
                name,
                address,
                notes,
            })
        })
        .collect::<Vec<_>>();

    return Some(fields);
}

pub async fn get_team_details(team_url: String) -> Option<TeamDetails> {
    let url = format!("https://www.psmf.cz{team_url}");
    let response = reqwest::get(url).await;
    let html_content = response.ok()?.text().await.ok()?;

    let document = Html::parse_document(&html_content);

    let next_match = parse_next_match(&document);
    let results = parse_results(&document).unwrap_or(vec![]);
    let matches = parse_matches(&document).unwrap_or(vec![]);

    return Some(TeamDetails {
        next_match,
        results,
        matches,
    });
}

fn parse_next_match(doc: &Html) -> Option<Match> {
    let closest_date_time_selector =
        Selector::parse(".component__closest-date time").expect("A valid selector");
    let date_str = doc
        .select(&closest_date_time_selector)
        .next()
        .unwrap()
        .attr("datetime")
        .unwrap()
        .to_owned();

    let closest_date_a_selector =
        Selector::parse(".component__closest-date a").expect("A valid selector");
    let court = doc
        .select(&closest_date_a_selector)
        .next()
        .unwrap()
        .attr("title")
        .unwrap()
        .to_owned();

    let closest_selector = Selector::parse(".component__closest").expect("A valid selector");
    let mut closest = doc
        .select(&closest_selector)
        .next()
        .unwrap()
        .child_elements();

    let home = closest.next().unwrap().text().collect::<String>();
    let guest = closest.next().unwrap().text().collect::<String>();

    //         // 2024-09-11 19:30
    //         let date = dateString?.parseDate(format: "yyyy-MM-dd HH:mm")

    return Some(Match {
        round: None,
        date: date_str,
        court,
        home_team: home,
        guest_team: guest,
        score: None,
    });
}

fn parse_results(doc: &Html) -> Option<Vec<Match>> {
    let old_table_selector =
        Selector::parse("table.component__table.games-old-table").expect("A valid selector");
    let table = doc.select(&old_table_selector);

    let rows = table
        .skip(1)
        .map(|row| {
            //let date_str = row.first_child().unwrap().
            //let dateString = try $0.child(0).text(trimAndNormaliseWhitespace: true)
            //let timeString = try $0.child(1).text(trimAndNormaliseWhitespace: true)

            //let date = "\(dateString.dropFirst(3)) \(timeString)".parseDate(
            //    format: "dd.MM.yy HH:mm"
            //)

            //return Match.init(
            //    round: Int(try $0
            //        .child(4)
            //        .text(trimAndNormaliseWhitespace: true)
            //        .dropLast()) ?? -1,
            //    date: date ?? Date.distantPast,
            //    court: try $0.child(2).text(trimAndNormaliseWhitespace: true),
            //    homeTeam: try $0
            //        .child(3)
            //        .child(0)
            //        .text(trimAndNormaliseWhitespace: true),
            //    guestTeam: try $0
            //        .child(3)
            //        .child(1)
            //        .text(trimAndNormaliseWhitespace: true),
            //    score: try $0
            //        .select("td.is-result")
            //        .text(trimAndNormaliseWhitespace: true)
            //)
            Match {
                round: todo!(),
                date: todo!(),
                court: todo!(),
                home_team: todo!(),
                guest_team: todo!(),
                score: todo!(),
            }
        })
        .collect::<Vec<_>>();

    return Some(rows);
}

fn parse_matches(doc: &Html) -> Option<Vec<Match>> {
    let new_table_selector =
        Selector::parse("table.component__table.games-new-table tr").expect("A valid selector");
    let table = doc.select(&new_table_selector).skip(1);

    let rows = table
        .map(|row| {
            let date_str = row.child_elements().next().unwrap().text().collect::<String>().trim().to_owned();
            //                 let dateString = try $0.child(0).text(trimAndNormaliseWhitespace: true)
            //                 let timeString = try $0.child(1).text(trimAndNormaliseWhitespace: true)

            //                 let date = "\(dateString.dropFirst(3)) \(timeString)".parseDate(
            //                     format: "dd.MM.yy HH:mm"
            //                 )

            //                 return Match.init(
            //                     round: Int(try $0
            //                         .child(4)
            //                         .text(trimAndNormaliseWhitespace: true)
            //                         .dropLast()) ?? -1,
            //                     date: date ?? Date.distantPast,
            //                     court: try $0.child(2).text(trimAndNormaliseWhitespace: true),
            //                     homeTeam: try $0
            //                         .child(3)
            //                         .child(0)
            //                         .text(trimAndNormaliseWhitespace: true),
            //                     guestTeam: try $0
            //                         .child(3)
            //                         .child(2) // here we have to skip dress info
            //                         .text(trimAndNormaliseWhitespace: true),
            //                     score: nil
            //                 )
            Match {
                date: todo!(),
                round: todo!(),
                court: todo!(),
                home_team: todo!(),
                guest_team: todo!(),
                score: todo!(),
            }
        })
        .collect::<Vec<_>>();

    return Some(rows);
}
