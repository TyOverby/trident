use super::*;

pub enum Command {
    MoveCarrier {
        carrier_id: CarrierId,
        to: StarId,
    }
}

pub fn distance(from: (i16, i16), to: (i16, i16)) -> i16 {
    (from.0 - to.0 + from.1 - to.1).abs()
}

pub fn transition(state: &mut GameState, player: PlayerId, command: Command) -> Result<(), String> {
    match command {
        Command::MoveCarrier {carrier_id, to} => {
            let (from, to, ticks_to_reach) = {
                let carrier = try!(state.carrier(carrier_id));
                let owner = try!(state.player(player));
                let destination = try!(state.star(to));

                if carrier.owner != owner.id {
                    return Err(format!("attempt to move {:?} does not belong to {:?}", carrier.id, player));
                }

                let carrier_star = if let Either::Left(star) = carrier.at {
                    try!(state.star(star))
                } else {
                    return Err(format!("tried to move carrier {:?} that is in transit", carrier.id));
                };

                let distance = distance(carrier_star.location, destination.location);
                // TODO: check distance calculation
                (carrier_star.id, destination.id, distance)
            };

            let mut carrier = try!(state.carrier_mut(carrier_id));
            carrier.at = Either::Right(Travel { from: from, to: to, ticks_to_reach: ticks_to_reach});
        }
    }
    Ok(())
}
