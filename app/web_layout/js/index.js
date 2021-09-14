import FeaApp from "./components/fea-app.js";
import FeaAppMenuBar from "./components/fea-app-menu-bar.js";
import FeaAppToolBar from "./components/fea-app-tool-bar.js";

import FeaRenderer from "./components/fea-renderer.js";

import FeaPreprocessorMenu from "./components/fea-preprocessor-menu.js";
import FeaPreprocessorMenuButtons from "./components/fea-preprocessor-menu-buttons.js";

import AddButton from "./components/common/crud-buttons/add-button.js";
import UpdateButton from "./components/common/crud-buttons/update-button.js";
import DeleteButton from "./components/common/crud-buttons/delete-button.js";

import AssignButton from "./components/common/assign-button.js";
import BeamSectionOrientationButton from "./components/common/beam-section-orientation-button.js";

import FeaGeometryMenu from "./components/fea-geometry/fea-geometry-menu.js";
import FeaGeometryMenuButtons from "./components/fea-geometry/fea-geometry-menu-buttons.js";
import FeaGeometryPointMenu from "./components/fea-geometry/point/fea-geometry-point-menu.js";
import FeaGeometryPointMenuButtons from "./components/fea-geometry/point/fea-geometry-point-menu-buttons.js";
import FeaGeometryLineMenu from "./components/fea-geometry/line/fea-geometry-line-menu.js";
import FeaGeometryLineMenuButtons from "./components/fea-geometry/line/fea-geometry-line-menu-buttons.js";
import FeaGeometryAddPointMenu from "./components/fea-geometry/point/fea-geometry-add-point-menu.js";
import FeaGeometryUpdatePointMenu from "./components/fea-geometry/point/fea-geometry-update-point-menu.js";
import FeaGeometryDeletePointMenu from "./components/fea-geometry/point/fea-geometry-delete-point-menu.js";
import FeaGeometryAddLineMenu from "./components/fea-geometry/line/fea-geometry-add-line-menu.js";
import FeaGeometryUpdateLineMenu from "./components/fea-geometry/line/fea-geometry-update-line-menu.js";
import FeaGeometryDeleteLineMenu from "./components/fea-geometry/line/fea-geometry-delete-line-menu.js";

import FeaMaterialMenu from "./components/fea-material/fea-material-menu.js";
import FeaMaterialMenuButtons from "./components/fea-material/fea-material-menu-buttons.js";
import FeaMaterialAddMaterialMenu from "./components/fea-material/fea-material-add-material-menu.js";
import FeaMaterialUpdateMaterialMenu from "./components/fea-material/fea-material-update-material-menu.js";
import FeaMaterialDeleteMaterialMenu from "./components/fea-material/fea-material-delete-material-menu.js";

import FeaSectionMenu from "./components/fea-section/fea-section-menu.js";
import FeaSectionMenuButtons from "./components/fea-section/fea-section-menu-buttons.js";

import FeaSectionTrussMenu from "./components/fea-section/truss/fea-section-truss-menu.js";
import FeaSectionTrussMenuButtons from "./components/fea-section/truss/fea-section-truss-menu-buttons.js";
import FeaSectionAddTrussMenu from "./components/fea-section/truss/fea-section-add-truss-menu.js";
import FeaSectionUpdateTrussMenu from "./components/fea-section/truss/fea-section-update-truss-menu.js";
import FeaSectionDeleteTrussMenu from "./components/fea-section/truss/fea-section-delete-truss-menu.js";

import FeaSectionBeamMenu from "./components/fea-section/beam/fea-section-beam-menu.js";
import FeaSectionBeamMenuButtons from "./components/fea-section/beam/fea-section-beam-menu-buttons.js";
import FeaSectionAddBeamMenu from "./components/fea-section/beam/fea-section-add-beam-menu.js";
import FeaSectionUpdateBeamMenu from "./components/fea-section/beam/fea-section-update-beam-menu.js";
import FeaSectionDeleteBeamMenu from "./components/fea-section/beam/fea-section-delete-beam-menu.js";

import FeaPropertiesMenu from "./components/fea-properties/fea-properties-menu.js";
import FeaPropertiesMenuButtons from "./components/fea-properties/fea-properties-menu-buttons.js";
import FeaPropertiesAddPropertiesMenu from "./components/fea-properties/fea-properties-add-properties-menu.js";
import FeaPropertiesUpdatePropertiesMenu from "./components/fea-properties/fea-properties-update-properties-menu.js";
import FeaPropertiesDeletePropertiesMenu from "./components/fea-properties/fea-properties-delete-properties-menu.js";
import FeaPropertiesAssignPropertiesMenu from "./components/fea-properties/fea-properties-assign-properties-menu.js";
import FeaPropertiesBeamSectionOrientationMenu from "./components/fea-properties/fea-properties-beam-section-orientation-menu.js";

import FeaLoadMenu from "./components/fea-load/fea-load-menu.js";
import FeaLoadMenuButtons from "./components/fea-load/fea-load-menu-buttons.js";
import FeaLoadAddConcentratedLoadMenu from "./components/fea-load/fea-load-add-concentrated-load-menu.js";
import FeaLoadUpdateConcentratedLoadMenu from "./components/fea-load/fea-load-update-concentrated-load-menu.js";
import FeaLoadDeleteConcentratedLoadMenu from "./components/fea-load/fea-load-delete-concentrated-load-menu.js";
import FeaLoadAddDistributedLineLoadMenu from "./components/fea-load/fea-load-add-distributed-line-load-menu.js";
import FeaLoadUpdateDistributedLineLoadMenu from "./components/fea-load/fea-load-update-distributed-line-load-menu.js";
import FeaLoadDeleteDistributedLineLoadMenu from "./components/fea-load/fea-load-delete-distributed-line-load-menu.js";

import FeaBoundaryConditionMenu from "./components/fea-boundary-condition/fea-boundary-condition-menu.js";
import FeaBoundaryConditionMenuButtons from "./components/fea-boundary-condition/fea-boundary-condition-menu-buttons.js";
import FeaBoundaryConditionAddBoundaryConditionMenu from "./components/fea-boundary-condition/fea-boundary-condition-add-boundary-condition-menu.js";
import FeaBoundaryConditionUpdateBoundaryConditionMenu from "./components/fea-boundary-condition/fea-boundary-condition-update-boundary-condition-menu.js";
import FeaBoundaryConditionDeleteBoundaryConditionMenu from "./components/fea-boundary-condition/fea-boundary-condition-delete-boundary-condition-menu.js";

import FeaAnalysisMenu from "./components/fea-analysis/fea-analysis-menu.js";

customElements.define("fea-app", FeaApp);
customElements.define("fea-app-menu-bar", FeaAppMenuBar);
customElements.define("fea-app-tool-bar", FeaAppToolBar);

customElements.define("fea-renderer", FeaRenderer);

customElements.define("fea-preprocessor-menu", FeaPreprocessorMenu);
customElements.define("fea-preprocessor-menu-buttons", FeaPreprocessorMenuButtons);

customElements.define("add-button", AddButton);
customElements.define("update-button", UpdateButton);
customElements.define("delete-button", DeleteButton);

customElements.define("assign-button", AssignButton);
customElements.define("beam-section-orientation-button", BeamSectionOrientationButton);

customElements.define("fea-geometry-menu", FeaGeometryMenu);
customElements.define("fea-geometry-menu-buttons", FeaGeometryMenuButtons);
customElements.define("fea-geometry-point-menu", FeaGeometryPointMenu);
customElements.define("fea-geometry-point-menu-buttons", FeaGeometryPointMenuButtons);
customElements.define("fea-geometry-line-menu", FeaGeometryLineMenu);
customElements.define("fea-geometry-line-menu-buttons", FeaGeometryLineMenuButtons);
customElements.define("fea-geometry-add-point-menu", FeaGeometryAddPointMenu);
customElements.define("fea-geometry-update-point-menu", FeaGeometryUpdatePointMenu);
customElements.define("fea-geometry-delete-point-menu", FeaGeometryDeletePointMenu);
customElements.define("fea-geometry-add-line-menu", FeaGeometryAddLineMenu);
customElements.define("fea-geometry-update-line-menu", FeaGeometryUpdateLineMenu);
customElements.define("fea-geometry-delete-line-menu", FeaGeometryDeleteLineMenu);

customElements.define("fea-material-menu", FeaMaterialMenu);
customElements.define("fea-material-menu-buttons", FeaMaterialMenuButtons);
customElements.define("fea-material-add-material-menu", FeaMaterialAddMaterialMenu);
customElements.define("fea-material-update-material-menu", FeaMaterialUpdateMaterialMenu);
customElements.define("fea-material-delete-material-menu", FeaMaterialDeleteMaterialMenu);

customElements.define("fea-section-menu", FeaSectionMenu);
customElements.define("fea-section-menu-buttons", FeaSectionMenuButtons);

customElements.define("fea-section-truss-menu", FeaSectionTrussMenu);
customElements.define("fea-section-truss-menu-buttons", FeaSectionTrussMenuButtons);
customElements.define("fea-section-add-truss-menu", FeaSectionAddTrussMenu);
customElements.define("fea-section-update-truss-menu", FeaSectionUpdateTrussMenu);
customElements.define("fea-section-delete-truss-menu", FeaSectionDeleteTrussMenu);

customElements.define("fea-section-beam-menu", FeaSectionBeamMenu);
customElements.define("fea-section-beam-menu-buttons", FeaSectionBeamMenuButtons);
customElements.define("fea-section-add-beam-menu", FeaSectionAddBeamMenu);
customElements.define("fea-section-update-beam-menu", FeaSectionUpdateBeamMenu);
customElements.define("fea-section-delete-beam-menu", FeaSectionDeleteBeamMenu);

customElements.define("fea-properties-menu", FeaPropertiesMenu);
customElements.define("fea-properties-menu-buttons", FeaPropertiesMenuButtons);
customElements.define("fea-properties-add-properties-menu", FeaPropertiesAddPropertiesMenu);
customElements.define("fea-properties-update-properties-menu", FeaPropertiesUpdatePropertiesMenu);
customElements.define("fea-properties-delete-properties-menu", FeaPropertiesDeletePropertiesMenu);
customElements.define("fea-properties-assign-properties-menu", FeaPropertiesAssignPropertiesMenu);
customElements.define("fea-properties-beam-section-orientation-menu", FeaPropertiesBeamSectionOrientationMenu);

customElements.define("fea-load-menu", FeaLoadMenu);
customElements.define("fea-load-menu-buttons", FeaLoadMenuButtons);
customElements.define("fea-load-add-concentrated-load-menu", FeaLoadAddConcentratedLoadMenu);
customElements.define("fea-load-update-concentrated-load-menu", FeaLoadUpdateConcentratedLoadMenu);
customElements.define("fea-load-delete-concentrated-load-menu", FeaLoadDeleteConcentratedLoadMenu);
customElements.define("fea-load-add-distributed-line-load-menu", FeaLoadAddDistributedLineLoadMenu);
customElements.define("fea-load-update-distributed-line-load-menu", FeaLoadUpdateDistributedLineLoadMenu);
customElements.define("fea-load-delete-distributed-line-load-menu", FeaLoadDeleteDistributedLineLoadMenu);

customElements.define("fea-boundary-condition-menu", FeaBoundaryConditionMenu);
customElements.define("fea-boundary-condition-menu-buttons", FeaBoundaryConditionMenuButtons);
customElements.define("fea-boundary-condition-add-boundary-condition-menu", FeaBoundaryConditionAddBoundaryConditionMenu);
customElements.define("fea-boundary-condition-update-boundary-condition-menu", FeaBoundaryConditionUpdateBoundaryConditionMenu);
customElements.define("fea-boundary-condition-delete-boundary-condition-menu", FeaBoundaryConditionDeleteBoundaryConditionMenu);

customElements.define("fea-analysis-menu", FeaAnalysisMenu);
