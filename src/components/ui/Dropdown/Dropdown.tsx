import { useState, useRef, useEffect } from "react";
import { ExpandMore, Check } from "@mui/icons-material";
import "./Dropdown.scss";

interface DropdownOption {
  value: string | number;
  label: string;
  disabled?: boolean;
  icon?: React.ReactNode;
}

interface DropdownProps {
  options: DropdownOption[];
  value?: string | number;
  placeholder?: string;
  disabled?: boolean;
  multiple?: boolean;
  searchable?: boolean;
  clearable?: boolean;
  size?: "sm" | "md" | "lg";
  variant?: "default" | "outline" | "ghost";
  error?: boolean;
  helperText?: string;
  label?: string;
  className?: string;
  onChange?: (value: string | number | (string | number)[]) => void;
  onSearch?: (query: string) => void;
}

const Dropdown = ({
  options,
  value,
  placeholder = "Select an option...",
  disabled = false,
  multiple = false,
  searchable = false,
  clearable = false,
  size = "md",
  variant = "default",
  error = false,
  helperText,
  label,
  className = "",
  onChange,
  onSearch,
}: DropdownProps) => {
  const [isOpen, setIsOpen] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedValues, setSelectedValues] = useState<(string | number)[]>(
    multiple
      ? Array.isArray(value)
        ? value
        : value
          ? [value]
          : []
      : value
        ? [value]
        : [],
  );

  const dropdownRef = useRef<HTMLDivElement>(null);
  const searchInputRef = useRef<HTMLInputElement>(null);

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        setIsOpen(false);
        setSearchQuery("");
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  // Focus search input when dropdown opens
  useEffect(() => {
    if (isOpen && searchable && searchInputRef.current) {
      searchInputRef.current.focus();
    }
  }, [isOpen, searchable]);

  // Filter options based on search query
  const filteredOptions = searchQuery
    ? options.filter((option) =>
        option.label.toLowerCase().includes(searchQuery.toLowerCase()),
      )
    : options;

  const handleToggle = () => {
    if (disabled) return;
    setIsOpen(!isOpen);
  };

  const handleOptionSelect = (optionValue: string | number) => {
    let newSelectedValues: (string | number)[];

    if (multiple) {
      if (selectedValues.includes(optionValue)) {
        newSelectedValues = selectedValues.filter((v) => v !== optionValue);
      } else {
        newSelectedValues = [...selectedValues, optionValue];
      }
    } else {
      newSelectedValues = [optionValue];
      setIsOpen(false);
    }

    setSelectedValues(newSelectedValues);
    onChange?.(multiple ? newSelectedValues : newSelectedValues[0]);
  };

  const handleClear = (e: React.MouseEvent) => {
    e.stopPropagation();
    setSelectedValues([]);
    onChange?.(multiple ? [] : "");
  };

  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const query = e.target.value;
    setSearchQuery(query);
    onSearch?.(query);
  };

  const getSelectedLabel = () => {
    if (selectedValues.length === 0) return placeholder;

    if (multiple) {
      if (selectedValues.length === 1) {
        const option = options.find((opt) => opt.value === selectedValues[0]);
        return option?.label || "";
      }
      return `${selectedValues.length} items selected`;
    }

    const option = options.find((opt) => opt.value === selectedValues[0]);
    return option?.label || "";
  };

  const dropdownClasses = [
    "dropdown",
    `dropdown--${size}`,
    `dropdown--${variant}`,
    error && "dropdown--error",
    disabled && "dropdown--disabled",
    isOpen && "dropdown--open",
    className,
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <div className="dropdown-wrapper">
      {label && <label className="dropdown__label">{label}</label>}

      <div ref={dropdownRef} className={dropdownClasses}>
        <button
          type="button"
          className="dropdown__trigger"
          onClick={handleToggle}
          disabled={disabled}
          aria-expanded={isOpen}
          aria-haspopup="listbox"
        >
          <span className="dropdown__value">{getSelectedLabel()}</span>

          <div className="dropdown__icons">
            {clearable && selectedValues.length > 0 && (
              <button
                type="button"
                className="dropdown__clear"
                onClick={handleClear}
                aria-label="Clear selection"
              >
                ×
              </button>
            )}
            <ExpandMore
              className={`dropdown__chevron ${isOpen ? "dropdown__chevron--rotated" : ""}`}
              fontSize="small"
            />
          </div>
        </button>

        {isOpen && (
          <div className="dropdown__menu">
            {searchable && (
              <div className="dropdown__search">
                <input
                  ref={searchInputRef}
                  type="text"
                  className="dropdown__search-input"
                  placeholder="Search..."
                  value={searchQuery}
                  onChange={handleSearchChange}
                />
              </div>
            )}

            <ul className="dropdown__list" role="listbox">
              {filteredOptions.length === 0 ? (
                <li className="dropdown__item dropdown__item--empty">
                  No options found
                </li>
              ) : (
                filteredOptions.map((option) => {
                  const isSelected = selectedValues.includes(option.value);

                  return (
                    <li key={option.value} className="dropdown__item-wrapper">
                      <button
                        type="button"
                        className={`dropdown__item ${
                          isSelected ? "dropdown__item--selected" : ""
                        } ${option.disabled ? "dropdown__item--disabled" : ""}`}
                        onClick={() =>
                          !option.disabled && handleOptionSelect(option.value)
                        }
                        disabled={option.disabled}
                        role="option"
                        aria-selected={isSelected}
                      >
                        {multiple && (
                          <div className="dropdown__checkbox">
                            {isSelected && <Check fontSize="small" />}
                          </div>
                        )}

                        {option.icon && (
                          <span className="dropdown__item-icon">
                            {option.icon}
                          </span>
                        )}

                        <span className="dropdown__item-label">
                          {option.label}
                        </span>

                        {!multiple && isSelected && (
                          <Check
                            className="dropdown__item-check"
                            fontSize="small"
                          />
                        )}
                      </button>
                    </li>
                  );
                })
              )}
            </ul>
          </div>
        )}
      </div>

      {helperText && (
        <div
          className={`dropdown__helper ${error ? "dropdown__helper--error" : ""}`}
        >
          {helperText}
        </div>
      )}
    </div>
  );
};

export default Dropdown;
