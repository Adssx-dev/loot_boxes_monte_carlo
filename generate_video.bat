@echo off
rem I don't know why, starting this .bat directly does not work, instead you should copy the commands to a terminal
rm data/video.video.mp4
ffmpeg -framerate 30 -i data/images/%04d.png data/video/video.mp4;
