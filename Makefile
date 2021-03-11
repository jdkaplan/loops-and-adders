build:
	cargo run --release

deps:
	cargo install cargo-watch

watch:
	cargo-watch -x run

video: build stitch

stitch:
	ffmpeg \
		-y \
		-r 60 \
		-f image2 \
		-i output/frames/%d.png \
		output/video.mp4
