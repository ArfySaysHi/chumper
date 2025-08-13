import { SyntheticEvent } from "react";
import "./IconButton.scss";

interface IconButtonProps {
  children: React.ReactNode;
  className?: string;
  onClick: (e: SyntheticEvent) => void;
  variant?: "primary" | "secondary" | "tertiary";
}

const IconButton = ({
  children,
  className = "",
  onClick,
  variant = "primary",
  ...props
}: IconButtonProps) => {
  return (
    <button
      className={`icon-button icon-button--${variant}`}
      onClick={onClick}
      {...props}
    >
      {children}
    </button>
  );
};

export default IconButton;
