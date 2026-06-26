import { useEffect, useState, useCallback } from "react";
import Particles from "@tsparticles/react";
import { type Container } from "@tsparticles/engine";
import { loadSlim } from "@tsparticles/slim";
import { initParticlesEngine } from "@tsparticles/react";
import { useGlobalStore } from "@/stores/useGlobalStore";

interface ParticleBackgroundProps {
    className?: string;
}

function ParticleBackground({
    className = "",
}: ParticleBackgroundProps): JSX.Element {
    const theme = useGlobalStore((s) => s.theme);
    const [ready, setReady] = useState(false);

    const isDark = theme === "dark";

    useEffect(() => {
        initParticlesEngine(async (engine) => {
            await loadSlim(engine);
        }).then(() => {
            setReady(true);
        });
    }, []);

    const particlesLoaded = useCallback(
        async (_container: Container | undefined) => {
            // 粒子加载完成回调
        },
        [],
    );

    if (!ready) {
        return <></>;
    }

    return (
        <Particles
            className={className}
            id="particle-background"
            particlesLoaded={particlesLoaded}
            options={{
                fullScreen: {
                    enable: false,
                },
                background: {
                    color: {
                        value: "transparent",
                    },
                },
                fpsLimit: 60,
                interactivity: {
                    events: {
                        onHover: {
                            enable: true,
                            mode: "grab",
                        },
                    },
                    modes: {
                        grab: {
                            distance: 160,
                            links: {
                                opacity: 0.3,
                                color: "#6366f1",
                            },
                        },
                    },
                },
                particles: {
                    color: {
                        value: "#6366f1",
                    },
                    links: {
                        color: "#6366f1",
                        distance: 150,
                        enable: true,
                        opacity: isDark ? 0.15 : 0.1,
                        width: 1,
                    },
                    move: {
                        direction: "none",
                        enable: true,
                        outModes: {
                            default: "bounce",
                        },
                        random: true,
                        speed: 1.2,
                        straight: false,
                    },
                    number: {
                        density: {
                            enable: true,
                        },
                        value: 60,
                    },
                    opacity: {
                        value: isDark ? 0.35 : 0.25,
                    },
                    shape: {
                        type: "circle",
                    },
                    size: {
                        value: { min: 1, max: 3 },
                    },
                },
                detectRetina: true,
            }}
        />
    );
}

export default ParticleBackground;