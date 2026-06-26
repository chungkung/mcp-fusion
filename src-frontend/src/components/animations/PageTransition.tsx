import { type ReactNode } from "react";
import { motion, type Variants } from "framer-motion";

interface PageTransitionProps {
    children: ReactNode;
    className?: string;
    variants?: Variants;
}

const defaultVariants: Variants = {
    initial: {
        opacity: 0,
        y: 12,
    },
    animate: {
        opacity: 1,
        y: 0,
        transition: {
            duration: 0.35,
            ease: [0.25, 0.46, 0.45, 0.94],
        },
    },
    exit: {
        opacity: 0,
        y: -12,
        transition: {
            duration: 0.25,
            ease: [0.25, 0.46, 0.45, 0.94],
        },
    },
};

function PageTransition({
    children,
    className = "",
    variants = defaultVariants,
}: PageTransitionProps): JSX.Element {
    return (
        <motion.div
            className={className}
            variants={variants}
            initial="initial"
            animate="animate"
            exit="exit"
        >
            {children}
        </motion.div>
    );
}

export default PageTransition;