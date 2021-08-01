.PHONY: build
build:
	cargo build

.PHONY: run
run:
	@./target/debug/quick-chess

.PHONY: play
play:
	@cutechess-cli -debug \
		-engine cmd=./target/debug/quick-chess \
		-engine cmd=./target/debug/quick-chess \
		-each proto=uci tc=180/2 -rounds 1 \
		-epdout games.log > debug.log

.PHONY: logs
logs:
	@tail -f /tmp/quick-chess/game.log
