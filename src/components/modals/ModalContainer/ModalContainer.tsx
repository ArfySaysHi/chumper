import { ReactNode } from "react";
import { createPortal } from "react-dom";
import { motion, Transition } from "framer-motion";
import "./ModalContainer.scss";

interface ModalContainerProps {
  onClose: () => void;
  children: ReactNode;
}

const transitionExitFast: Transition = { duration: 0.2 };
const transitionExit: Transition = { duration: 0.3, ease: "easeOut" };

const modalVariants = {
  initial: { opacity: 0, scale: 0.9 },
  animate: {
    opacity: 1,
    scale: 1,
    transition: transitionExit,
  },
  exit: { opacity: 0, scale: 0.9, transition: transitionExitFast },
};

const ModalContainer = ({ onClose, children }: ModalContainerProps) => {
  return createPortal(
    <div className="modal-container" onClick={onClose}>
      <motion.div
        className="modal-container__overlay"
        initial={{ opacity: 0 }}
        animate={{
          opacity: 1,
          transition: { duration: 0.3, ease: "easeOut" },
        }}
        exit={{ opacity: 0, transition: { duration: 0.2 } }}
      />
      <motion.div
        role="dialog"
        aria-modal="true"
        className="modal-container__content"
        onClick={(e) => e.stopPropagation()}
        variants={modalVariants}
        initial="initial"
        animate="animate"
        exit="exit"
      >
        {children}
      </motion.div>
    </div>,
    document.getElementById("portal-root") || document.body,
  );
};

export default ModalContainer;
