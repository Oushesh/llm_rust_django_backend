"""
import whisper

model = whisper.load_model("base")
result = model.transcribe("y2mate.bz - Jeff Bezos on colonizing the Moon _ Lex Fridman Podcast Clips.mp4")

with open("transcription.txt","w") as f:
    f.write(result["text"])



#139 mb model
"""

import whisper

model = whisper.load_model("base")

# Check if language identification is supported by the model
if hasattr(model, 'detect_language'):
    # load audio and pad/trim it to fit 30 seconds
    audio = whisper.load_audio("output_005.mp3")
    audio = whisper.pad_or_trim(audio)

    # make log-Mel spectrogram and move to the same device as the model
    mel = whisper.log_mel_spectrogram(audio).to(model.device)

    # Check if language identification is supported by the model
    if hasattr(model, 'detect_language'):
        # detect the spoken language
        _, probs = model.detect_language(mel)
        print(f"Detected language: {max(probs, key=probs.get)}")
    else:
        print("Language identification is not supported by this model.")

    fp16 = False
    # decode the audio
    options = whisper.DecodingOptions(fp16 = False,language="english")
    result = whisper.decode(model, mel, options)

    # print the recognized text
    print(result.text)
else:
    print("Language identification is not supported by this model.")
