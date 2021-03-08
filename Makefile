build:
	cargo run --release

watch:
	fd . | entr -cr cargo run

video: build stitch

stitch:
	ffmpeg \
		-y \
		-r 60 \
		-f image2 \
		-i output/frames/%d.png \
		output/video.mp4
