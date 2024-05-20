import argparse
from gradio_client import Client
import sys
import codecs

sys.stdout = codecs.getwriter("utf-8")(sys.stdout.detach()) # utf8 output for rust subprocesses

# Create the parser
parser = argparse.ArgumentParser()

# Add the arguments
parser.add_argument('Prompt', metavar='Prompt', type=str, help='the prompt for the image')
parser.add_argument('--status', action='store_true', help='check the status of the client')

# Parse the arguments
args = parser.parse_args()

client = Client("https://ap123-illusiondiffusion.hf.space/--replicas/w3ut2/", verbose=False)

# If --status is provided, exit after connecting to the client
if args.status:
    sys.exit(0)

result = client.predict(
    # "https://raw.githubusercontent.com/gradio-app/gradio/main/test/test_files/bus.png",  # filepath  in 'Input Illusion' Image component
    "https://upload.wikimedia.org/wikipedia/en/9/9a/Trollface_non-free.png",
    args.Prompt,  # str  in 'Prompt' Textbox component
    "low quality",  # str  in 'Negative Prompt' Textbox component
    7.5,  # float (numeric value between 0.0 and 50.0) in 'Guidance Scale' Slider component
    1.0,  # float (numeric value between 0.0 and 5.0) in 'Illusion strength' Slider component
    0.0,  # float (numeric value between 0.0 and 1.0) in 'Start of ControlNet' Slider component
    1.0,  # float (numeric value between 0.0 and 1.0) in 'End of ControlNet' Slider component
    1.0,  # float (numeric value between 0.0 and 1.0) in 'Strength of the upscaler' Slider component
    -1,  # float (numeric value between -1 and 9999999999) in 'Seed' Slider component
    # "Euler",  # Literal['DPM++ Karras SDE', 'Euler']  in 'parameter_13' Dropdown component
    "DPM++ Karras SDE", 
    api_name="/inference"
)

# The result is a tuple, the first element is the image path
image_path = result[0]

print(image_path)