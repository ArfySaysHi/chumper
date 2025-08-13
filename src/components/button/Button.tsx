import './Button.scss';

interface ButtonProps {
    variant?: 'primary' | 'secondary';
    size?: 'sm' | 'md' | 'lg';
    children: React.ReactNode;
}

const Button = ({ variant = 'primary', size = 'md', children, ...props }: ButtonProps) => (
    <button
        className={`button button--${variant} button--${size}`}
        {...props}
    >
        {children}
    </button>
);

export default Button;
