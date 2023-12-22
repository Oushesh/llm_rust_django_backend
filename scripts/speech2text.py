import whisper

model = whisper.load("base")
result = model.transcribe("y2mate.bz - Jeff Bezos on colonizing the Moon _ Lex Fridman Podcast Clips.mp4")

with open("transcription.txt","w") as f:
    f.write(result["text"])