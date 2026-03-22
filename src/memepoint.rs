// use std::env;
// use std::fs::File;

// use tokio::task;
use serenity::all::{Context, GuildId, RoleId, UserId};
// use serenity::http::Http;
// use serenity::FutureExt;

pub async fn calc_meme_points(
    ctx: &Context,
    guild_id: GuildId,
    roles: Vec<RoleId>,
    user: UserId,
) -> Result<u32, Box<dyn std::error::Error>> {
    if guild_id.get() == 653952153005588490 {
        let mut score: u32 = 0;
        let mut rdr = csv::Reader::from_path("memerole/653952153005588490")?;
        let mut values: Vec<(u64, u32)> = vec![];
        let mut has_role: [bool; 5] = [false; 5];
        for result in rdr.records() {
            let record = result?;
            values.push((
                record[0]
                    .to_string()
                    .parse()
                    .expect("vro tried to string a number"),
                record[1]
                    .to_string()
                    .parse()
                    .expect("vro tried to string a number"),
            ));
        }
        let mut rdr = csv::Reader::from_path("memerole/653952153005588490yellow")?;
        let mut yellows: Vec<u64> = vec![];
        for result in rdr.records() {
            let record = result?;
            yellows.push(
                record[0]
                    .to_string()
                    .parse()
                    .expect("vro tried to string a number"),
            );
        }
        let mut rdr = csv::Reader::from_path("memerole/653952153005588490medal")?;
        let mut medals: Vec<u64> = vec![];
        for result in rdr.records() {
            let record = result?;
            medals.push(
                record[0]
                    .to_string()
                    .parse()
                    .expect("vro tried to string a number"),
            );
        }
        let mut rdr = csv::Reader::from_path("memerole/653952153005588490lewd")?;
        let mut lewds: Vec<u64> = vec![];
        for result in rdr.records() {
            let record = result?;
            lewds.push(
                record[0]
                    .to_string()
                    .parse()
                    .expect("vro tried to string a number"),
            );
        }
        let mut rdr = csv::Reader::from_path("memerole/653952153005588490delete")?;
        let mut deletes: Vec<(u64, u64)> = vec![];
        for result in rdr.records() {
            let record = result?;
            deletes.push((
                record[0]
                    .to_string()
                    .parse()
                    .expect("vro tried to string a number"),
                record[1]
                    .to_string()
                    .parse()
                    .expect("vro tried to string a number"),
            ));
        }
        for role in &roles {
            for delete in &deletes {
                if role.get() == delete.0 {
                    for (n, value) in values.iter().enumerate() {
                        if value.0 == delete.1 {
                            values.remove(n);
                            break;
                        }
                    }
                }
            }
        }
        'medal: for role in &roles {
            for medal in &medals {
                if role.get() == *medal {
                    score += 3;
                    break 'medal;
                }
            }
        }
        'lewd: for role in &roles {
            for lewd in &lewds {
                if role.get() == *lewd {
                    score += 2;
                    break 'lewd;
                }
            }
        }
        for role in &roles {
            match role.get() {
                1208905414159237172 => {
                    for (n, value) in values.clone().iter().enumerate() {
                        for yellow in yellows.clone() {
                            if value.0 == yellow {
                                values[n].1 += 1
                            }
                        }
                    }
                }
                823592784341893190 => has_role[0] = true,
                835100509169516564 => has_role[1] = true,
                915380497939578930 => has_role[2] = true,
                1062542877265231943 => has_role[3] = true,
                1374815808856789104 => has_role[4] = true,
                _ => (),
            }
        }
        for role in roles {
            for (n, value) in values.iter().enumerate() {
                if role.get() == value.0 {
                    score += value.1;
                    values.remove(n);
                    break;
                }
            }
        }
        println!("{:?}", has_role);
        if !has_role[0] && score >= 6 {
            let _ = ctx
                .http
                .add_member_role(
                    guild_id,
                    user,
                    RoleId::new(823592784341893190),
                    Some("decent"),
                )
                .await;
        };
        if !has_role[1] && score >= 12 {
            let _ = ctx
                .http
                .add_member_role(
                    guild_id,
                    user,
                    RoleId::new(835100509169516564),
                    Some("acceptable"),
                )
                .await;
        };
        if !has_role[2] && score >= 20 {
            let _ = ctx
                .http
                .add_member_role(
                    guild_id,
                    user,
                    RoleId::new(915380497939578930),
                    Some("good"),
                )
                .await;
        };
        if !has_role[3] && score >= 32 {
            let _ = ctx
                .http
                .add_member_role(
                    guild_id,
                    user,
                    RoleId::new(1062542877265231943),
                    Some("very good"),
                )
                .await;
        };
        if !has_role[4] && score >= 60 {
            let _ = ctx
                .http
                .add_member_role(
                    guild_id,
                    user,
                    RoleId::new(1374815808856789104),
                    Some("super good"),
                )
                .await;
        };
        println!("{}", score);
        Ok(score)
    } else {
        Ok(2)
    }
}

pub fn updateboard(
    guild: GuildId,
    user: UserId,
    score: u32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let name = "board/".to_owned() + &guild.to_string();
    let mut rdr = csv::Reader::from_path(name)?;
    let mut output: Vec<(String, String)> = vec![];
    let mut values: Vec<(u64, u32)> = vec![];
    output.push(("user".to_string(), "score".to_string()));
    let mut has_id = false;
    for result in rdr.records() {
        let record = result?;
        if record[0]
            .to_string()
            .parse::<u64>()
            .expect("vro tried to string a number")
            == user.get()
        {
            values.push((
                record[0]
                    .to_string()
                    .parse()
                    .expect("vro tried to string a number"),
                score,
            ));
            has_id = true;
        } else {
            values.push((
                record[0]
                    .to_string()
                    .parse()
                    .expect("vro tried to string a number"),
                record[1]
                    .to_string()
                    .parse()
                    .expect("vro tried to string a number"),
            ))
        }
    }
    if !has_id {
        values.push((user.get(), score));
    }
    values.sort_by(|a, b| b.1.cmp(&a.1));
    for value in values {
        output.push((value.0.to_string(), value.1.to_string()))
    }
    let name = "board/".to_owned() + &guild.to_string();
    let mut wtr = csv::Writer::from_path(name)?;
    for line in output {
        wtr.write_record([line.0, line.1])?;
    }
    Ok(true)
}

pub fn read_board(guildid: GuildId) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let name = "board/".to_owned() + &guildid.to_string();
    let mut rdr = csv::Reader::from_path(name)?;
    let mut output: Vec<(String, String)> = vec![];
    output.push(("195219971754819584".to_string(), "6667".to_string()));
    for (n, result) in rdr.records().enumerate() {
        println!("{:?}", result);
        let record = result?;
        if n == 19 {
            break;
        }
        output.push((record[0].to_string(), record[1].to_string()));
    }
    Ok(output)
}

pub fn calc_max_points(theory: bool) -> Result<u32, Box<dyn std::error::Error>> {
    let mut score: u32 = 0;
    let mut values: Vec<(u64, u32)> = vec![];
    let mut rdr = csv::Reader::from_path("memerole/653952153005588490")?;
    for result in rdr.records() {
        let record = result?;
        values.push((
            record[0]
                .to_string()
                .parse()
                .expect("vro tried to string a number"),
            record[1]
                .to_string()
                .parse()
                .expect("vro tried to string a number"),
        ));
    }
    let mut rdr = csv::Reader::from_path("memerole/653952153005588490yellow")?;
    let mut yellows: Vec<u64> = vec![];
    for result in rdr.records() {
        let record = result?;
        yellows.push(
            record[0]
                .to_string()
                .parse()
                .expect("vro tried to string a number"),
        );
    }
    let mut rdr = csv::Reader::from_path("memerole/653952153005588490delete")?;
    let mut deletes: Vec<(u64, u64)> = vec![];
    for result in rdr.records() {
        let record = result?;
        deletes.push((
            record[0]
                .to_string()
                .parse()
                .expect("vro tried to string a number"),
            record[1]
                .to_string()
                .parse()
                .expect("vro tried to string a number"),
        ));
    }
    score += 3;
    score += 2;
    if theory {
        for (n, value) in values.clone().iter().enumerate() {
            values[n].1 += 1;
            if value.1 >= 4 {
                values[n].1 += 1
            }
        }
    }
    for value in values.clone() {
        for delete in &deletes {
            if value.0 == delete.0 {
                for (n, value) in values.clone().iter().enumerate() {
                    if value.0 == delete.1 {
                        values.remove(n);
                    }
                }
            }
        }
    }
    for _yellow in &yellows {
        score += 1
    }
    for value in &values {
        score += value.1
    }
    Ok(score)
}

pub fn genboard(guild_id: GuildId) -> Vec<(String, String, bool)> {
    let mut output: Vec<(String, String, bool)> = vec![];
    let input: Vec<(String, String)> = read_board(guild_id).expect("awawa");
    let mut n: usize = 0;
    for text in input {
        n += 1;
        println!("{:?}", text);
        output.push((
            "".to_string(),
            n.to_string() + ". <@" + &text.0 + "> - " + &text.1,
            false,
        ));
    }
    output.push((
        "Maximum Meme Points Attainable Currently -".to_string(),
        calc_max_points(false)
            .expect("cant calc max score")
            .to_string(),
        false,
    ));
    output.push((
        "Maximum Meme Points Attainable Post-Major Quest 1 -".to_string(),
        calc_max_points(true)
            .expect("cant calc max score")
            .to_string(),
        false,
    ));
    output
}
