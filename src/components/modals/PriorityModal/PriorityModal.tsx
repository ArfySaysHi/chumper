import "./PriorityModal.scss";
import { useState } from "react";
import ModalContainer from "../ModalContainer/ModalContainer.tsx";
import PriorityModalMethod from "./PriorityModalMethod/PriorityModalMethod.tsx";

interface PriorityModalProps {
  onClose: () => void;
}

const PriorityModal = ({ onClose }: PriorityModalProps) => {
  const [currentStep, setCurrentStep] = useState("method");

  return (
    <ModalContainer onClose={onClose}>
      {currentStep === "method" && (
        <PriorityModalMethod setCurrentStep={setCurrentStep} />
      )}
      {currentStep === "priority" && <div>Priority</div>}
    </ModalContainer>
  );
};

export default PriorityModal;
