import argparse
from gradio_client import Client
import sys
import codecs

sys.stdout = codecs.getwriter("utf-8")(sys.stdout.detach()) # utf8 output for rust subprocesses

# Create the parser
parser = argparse.ArgumentParser()

# Add the arguments
parser.add_argument('--prompt', metavar='Prompt', type=str, help='the prompt for the image', required=False)
parser.add_argument('--image', metavar='Image', type=str, help='the input image link', required=False)
parser.add_argument('--status', action='store_true', help='check the status of the client', required=False)

# Parse the arguments
args = parser.parse_args()

# Check if the prompt and image arguments are provided
if (args.prompt is None or args.image is None) and not args.status:
    print("Error: the prompt and image arguments are required when --status is not set")
    sys.exit(1)

client = Client("https://ap123-illusiondiffusion.hf.space/--replicas/nctv1/", verbose=False)

# If --status is provided, exit after connecting to the client
if args.status:
    sys.exit(0)



# Use the provided image link
image_link = args.image

result = client.predict(
    image_link,
    args.prompt,  # str  in 'Prompt' Textbox component
    "low quality",  # str  in 'Negative Prompt' Textbox component
    7.5,  # float (numeric value between 0.0 and 50.0) in 'Guidance Scale' Slider component
    0.8,  # float (numeric value between 0.0 and 5.0) in 'Illusion strength' Slider component
    0.0,  # float (numeric value between 0.0 and 1.0) in 'Start of ControlNet' Slider component
    1.0,  # float (numeric value between 0.0 and 1.0) in 'End of ControlNet' Slider component
    1.0,  # float (numeric value between 0.0 and 1.0) in 'Strength of the upscaler' Slider component
    -1,  # float (numeric value between -1 and 9999999999) in 'Seed' Slider component
    "DPM++ Karras SDE", 
    api_name="/inference"
)

# The result is a tuple, the first element is the image path
image_path = result[0]

print(image_path)