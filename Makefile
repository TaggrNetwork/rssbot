enable_canister_logs:
	dfx canister --network ic update-settings rssbot --log-visibility public

deploy:
	dfx deploy --network ic

logs:
	dfx canister --ic logs rssbot
	dfx canister --network ic call --query rssbot info '("logs")'

stats:
	dfx canister --network ic call --query rssbot info '("stats")'

fixture:
	dfx canister --network ic call rssbot fixture

status:
	dfx canister --network ic status rssbot
