import "./Button.scss";

interface ButtonProps {
  className?: string;
  variant?: "primary" | "secondary" | "icon";
  destructive?: boolean;
  size?: "xs" | "md" | "lg";
  title?: string;
  children: React.ReactNode;
  onClick?: () => void;
}

const Button = ({
  className = "",
  variant = "primary",
  destructive = false,
  size = "md",
  title = "",
  children,
  ...props
}: ButtonProps) => (
  <button
    className={`button ${className} button--${variant} button--${size}${destructive ? " button--danger" : ""}`}
    title={title}
    {...props}
  >
    {children}
  </button>
);

export default Button;
