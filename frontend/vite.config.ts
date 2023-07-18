import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import tsconfigPaths from "vite-tsconfig-paths";
import dotenv from "dotenv";
dotenv.config({ path: "./.env" });
dotenv.config({ path: "../contract/neardev/dev-account.env" });
console.log(process.env.CONTRACT_NAME);
// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react(), tsconfigPaths()],
    define: {
        "process.env": {
            REACT_APP_CONTRACT_ID:
                process.env.CONTRACT_NAME || process.env.REACT_APP_CONTRACT_ID || "dev-1689354179430-79495688832247",
        },
        global: "window",
    },
});
