import { serve } from "https://deno.land/std@0.154.0/http/server.ts"
import { instantiate } from "./lib/rs_lib.generated.js"

const { handler } = await instantiate()

serve(handler)
