# mafia-lol-api
The backend for mafia-lol.

## mafia-lol
mafia-lol is a web app meant to help administrating mafia games.

## Plan
* Create lobby
* Join lobby automatically
* Select player name
* Select allowed roles
* Share lobby through short url
* Newly joined players select names
* Joined players see role pool
* Lobby creator can start game
* Roles get assigned to all player randomly
* Web app asks admin to make the town sleep
* Admin clicks next button to get instructions from web app
* Web app asks admin to wake up godfather and goon
* Admin wakes up godfather and goon
* Web app asks admin to read what can godfather and goon do
* Admin reads what can godfather and goon do
* Godfather and goon select who to shoot
* Admin enters mafia target into the web app
* Web app validates the action is legal (maybe special rule saying you cant target the same person twice)
* Admin clicks next to confirm (clicking next is not allowed if action is illegal) (optional bypass)
* Web app asks admin to wake up hooker
* Admin wakes up hooker
* Web app asks admin to read what can a hooker do
* Admin reads what can a hooker do
* Hooker selects who to silence for the night
* Admin enters the hooker's target
* Web app validates the action is legal (maybe special rule saying you cant target the same person twice)
* Admin clicks next to confirm (clicking next is not allowed if action is illegal) (optional bypass)
* ...
* Final role does its thing
* Web app evaluates all actions
* Web app asks admin to wake the town up
* Web app asks admin to read who died (if anyone)
* Web app asks admin to announce special effects (if any) (such as does vigilante have a gun)

