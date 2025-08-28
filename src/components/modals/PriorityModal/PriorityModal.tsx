import "./PriorityModal.scss";
import { useState } from "react";
import ModalContainer from "../ModalContainer/ModalContainer.tsx";
import PriorityModalMethod from "./PriorityModalMethod/PriorityModalMethod.tsx";

interface PriorityModalProps {
  onClose: () => void;
}

const PriorityModal = ({ onClose }: PriorityModalProps) => {
  const [currentStep, _] = useState("method");

  return (
    <ModalContainer onClose={onClose}>
      {currentStep && <PriorityModalMethod />}
    </ModalContainer>
  );
};

export default PriorityModal;
