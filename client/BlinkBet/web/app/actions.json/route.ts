import { ACTIONS_CORS_HEADERS, ActionsJson } from "@solana/actions";

export async function GET() {
  const payload: ActionsJson = {
    rules: [
      {
        pathPattern: "/blink",
        apiPath: "/api/hello",
      },
    ],
  };

  return Response.json(payload, {
    headers: ACTIONS_CORS_HEADERS
  });
};

export function OPTIONS() {GET};