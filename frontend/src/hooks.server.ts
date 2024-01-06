import { SvelteKitAuth } from "@auth/sveltekit";
import Cognito from "@auth/sveltekit/providers/cognito";
// import {AUTH_SECRET, COGNITO_CLIENT_ID, COGNITO_CLIENT_SECRET, COGNITO_ISSUER} from "$env/static/private"

export const handle = SvelteKitAuth({
  secret: "590582b87b8f34f064a1a82a873d56fcd4ec8f4dbaa9887370377f13ce9a02b5",
  providers: [Cognito({clientId: "5os1r30g3bj7q208ot5mfe9009", clientSecret: "ieekhh98gk32pfantjda29v3dhe3gc5vqh87gqdsu7ihre819m8", issuer: "https://cognito-idp.us-east-1.amazonaws.com/us-east-1_qlclDg1iF"})],
  callbacks: {
    async session({session }) {
      // Make call to server to get user details (name, premium sub)

      return session;
    }
  }
})
