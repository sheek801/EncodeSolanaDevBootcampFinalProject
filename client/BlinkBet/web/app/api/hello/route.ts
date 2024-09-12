import {ActionGetResponse} from "@solana/actions"

export async function GET(request: Request) {
  const response : ActionGetResponse = {
    icon: 
    description: "Select an SPL token and predict how much it will increase",
    title: "BetBlink",
    label: "10"
    error:



  }
  return new Response('Hello, from API!');
}
