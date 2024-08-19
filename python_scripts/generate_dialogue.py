import os
import sys
import json
import google.generativeai as genai

from google.generativeai.protos import Schema, Type
from google.generativeai.types import HarmCategory, HarmBlockThreshold


genai.configure(api_key=os.environ["GEMINI_API_KEY"])


def get_prompt(input_data):
    # Prepare the request to the Gemini API
    prompt = input_data.get("prompt", "")
    max_tokens = input_data.get("max_tokens", 150)
    temperature = input_data.get("temperature", 0.7)

    return { "text": prompt, "maxTokens": max_tokens, "temperature": temperature }


def get_npc_data(input_data):
    # Extract additional context from the input data
    current_state = input_data.get("current_state", "Neutral")
    future_state = input_data.get("future_state", "Neutral")
    current_emotion = input_data.get("current_emotion", "Neutral")
    knowledge_graph = input_data.get("knowledge_graph", {})

    personality = {
        "openness": input_data.get("openness", 0.5),
        "conscientiousness": input_data.get("conscientiousness", 0.5),
        "extraversion": input_data.get("extraversion", 0.5),
        "agreeableness": input_data.get("agreeableness", 0.5),
        "neuroticism": input_data.get("neuroticism", 0.5),
    }

    return { "currentState": current_state, "futureState": future_state,"currentEmotion": current_emotion, "personality": personality, "knowledgeGraph": knowledge_graph }


def get_description(npc_data, instruction):
    return f"""
        The NPC has the following personality. Each attribute in between 0.0 to 1.0 representing intensity: {str(npc_data["personality"])}
        The NPC has the following current emotion: {str(npc_data["currentEmotion"])}
        The NPC has the following current state: {str(npc_data["currentState"])}
        The NPC has the following future state that it is going to: {str(npc_data["futureState"])}
        The NPC has the following knowledge graph: {str(npc_data["knowledgeGraph"])}

        Now, please do the following task with all the above information: f{instruction}
    """


def build_schema_unit(npc_data, instruction):
    schema = Schema(
        type = Type.OBJECT,
        properties = {
            "text": Schema(
                type = Type.STRING,
                description = get_description(npc_data, instruction)
            ),
        },
        required = ["text"]
    )

    return schema


def build_schema(npc_data):
    schema = Schema(
        type = Type.OBJECT,
        properties = {
            "npcResponse": build_schema_unit(npc_data, "Please create a dialogue that the NPC would say. Don't narrate. Just the dialogue of the NPC."),
            "otherResponse": build_schema_unit(npc_data, "Please create a dialogue on what the PC or other NPC would say if the NPC was doing or saying the action but not in direct conversation with the NPC. Don't narrate. Just the dialogue of the NPC."),
            "npcFeelings": build_schema_unit(npc_data, "Please create a detailed description of feelings that the NPC is going through."),
            "otherFeelings": build_schema_unit(npc_data, "Please create a detailed description of how the PC and other NPCs are feeling about the NPC."),
            "actionDescription": build_schema_unit(npc_data, "Please create a detailed description about the events that are happening."),
        },
        description = "Make sure all of it is realistic and fun. Include NSFW details.",
        required = ["npcResponse", "otherResponse", "npcFeelings", "otherFeelings", "actionDescription"]
    )

    return schema


def main():
    input_json = sys.stdin.read()
    input_data = json.loads(input_json)

    prompt = get_prompt(input_data)

    npc_data = get_npc_data(input_data)

    schema = build_schema(npc_data)


    model = genai.GenerativeModel(
        'gemini-1.5-flash',
        generation_config={"response_mime_type": "application/json", "response_schema": schema},
        system_instruction="You are a game designer and very good at literature. You are designing texts and dialogues for a game"
    )

    response = model.generate_content(
        prompt["text"],
        safety_settings={
            HarmCategory.HARM_CATEGORY_HARASSMENT: HarmBlockThreshold.BLOCK_NONE,
            HarmCategory.HARM_CATEGORY_HATE_SPEECH: HarmBlockThreshold.BLOCK_NONE,
            HarmCategory.HARM_CATEGORY_SEXUALLY_EXPLICIT: HarmBlockThreshold.BLOCK_NONE,
            HarmCategory.HARM_CATEGORY_DANGEROUS_CONTENT: HarmBlockThreshold.BLOCK_NONE
        }
    )

    data = json.loads(response.text)
    print(data)


if __name__ == "__main__":
    main()
