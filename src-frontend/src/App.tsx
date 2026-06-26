import { RouterProvider } from "react-router-dom";
import { Toaster } from "react-hot-toast";

import { router } from "@/routes";
import { useGlobalStore } from "@/stores/useGlobalStore";

import "@/styles/globals.css";

function App(): JSX.Element {
    const theme = useGlobalStore((s) => s.theme);

    return (
        <>
            <RouterProvider router={router} />
            <Toaster
                position="bottom-right"
                toastOptions={{
                    duration: 3000,
                    style: {
                        borderRadius: "8px",
                        background:
                            theme === "dark" ? "#1e293b" : "#ffffff",
                        color: theme === "dark" ? "#e2e8f0" : "#0f172a",
                        border:
                            theme === "dark"
                                ? "1px solid #334155"
                                : "1px solid #e2e8f0",
                    },
                }}
            />
        </>
    );
}

export default App;