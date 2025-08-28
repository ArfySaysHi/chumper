import "./PriorityModalMethod.scss";
import IconInfoCard from "../../../ui/IconInfoCard/IconInfoCard.tsx";
import { Abc, PlusOne, Spa } from "@mui/icons-material";

const methods = [
  {
    id: "priority",
    title: "Priority System",
    description:
      "Classic approach - assign one priority level (A-E) to each category",
    icon: Abc,
  },
  {
    id: "sum-to-ten",
    title: "Sum-to-Ten",
    description:
      "Flexible point allocation - distribute 10 points across categories",
    icon: PlusOne,
  },
  {
    id: "karma-gen",
    title: "Karma Generation",
    description: "Point-buy system using karma for complete customization",
    icon: Spa,
  },
];

const PriorityModalMethod = () => {
  return (
    <div className="priority-modal-method">
      <div className="priority-modal-method__header">
        <h2 className="priority-modal-method__header__title">
          Character Generation
        </h2>
        <span className="priority-modal-method__header__divider"></span>
        <p className="priority-modal-method__header__subheader">
          Select your preferred character creation method
        </p>
      </div>
      <div className="priority-modal-method__options">
        {methods.map((m) => (
          <IconInfoCard title={m.title} description={m.description}>
            <m.icon />
          </IconInfoCard>
        ))}
      </div>
    </div>
  );
};

export default PriorityModalMethod;
