import "./Loading.scss";

interface LoadingProps {
  text?: string | null;
  inline?: boolean;
  className?: string;
}

const Loading = ({ text = null, inline = false, className }: LoadingProps) => {
  return (
    <div
      className={`loading ${inline ? "loading--inline" : "loading--center"} ${className ?? ""}`}
    >
      <div className="loading__spinner" aria-hidden="true" />
      {text !== null && <div className="loading__text">{text}</div>}
    </div>
  );
};

export default Loading;
