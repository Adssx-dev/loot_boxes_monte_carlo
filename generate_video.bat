@echo off
rm data/video.video.mp4
ffmpeg -framerate 30 -i data/images/%04d.png data/video/video.mp4;
