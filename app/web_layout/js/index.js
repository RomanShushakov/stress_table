import HidingContentButton from "./common_components/hiding-content-button.js";
customElements.define("hiding-content-button", HidingContentButton);

import FeaApp from "./components/fea-app.js";
import FeaAppTitleBar from "./components/fea-app-title-bar.js";

import FeaPreprocessor from "./components/fea-preprocessor.js";
import FeaPreprocessorMenuButtons from "./components/fea-preprocessor-menu-buttons.js";

import FeaGeometry from "./components/fea-geometry.js";

import FeaGeometryMenu from "./components/fea-geometry/fea-geometry-menu.js";
import FeaGeometryMenuButtons from "./components/fea-geometry/fea-geometry-menu-buttons.js";
import FeaGeometryPointMenu from "./components/fea-geometry/point/fea-geometry-point-menu.js";
import FeaGeometryPointMenuButtons from "./components/fea-geometry/point/fea-geometry-point-menu-buttons.js";
import FeaGeometryLineMenu from "./components/fea-geometry/line/fea-geometry-line-menu.js";
import FeaGeometryLineMenuButtons from "./components/fea-geometry/line/fea-geometry-line-menu-buttons.js";
import FeaGeometryAddPointMenu from "./components/fea-geometry/point/fea-geometry-add-point-menu.js";
import FeaGeometryUpdatePointMenu from "./components/fea-geometry/point/fea-geometry-update-point-menu.js";


import FeaProperties from "./components/fea-properties.js";
import FeaMesh from "./components/fea-mesh.js";
import FeaAnalysis from "./components/fea-analysis.js";
import FeaBoundaryCondition from "./components/fea-boundary-condition.js";
import FeaLoad from "./components/fea-load.js";
import FeaRenderer from "./components/fea-renderer.js";
import FeaPostprocessor from "./components/fea-postprocessor.js";
import FeaPostprocessorMenu from "./components/fea-postprocessor-menu.js";
import FeaPlotDisplacements from "./components/fea-plot-displacements.js";
import FeaPlotStresses from "./components/fea-plot-stresses.js";
import FeaPlotStrains from "./components/fea-plot-strains.js";
import FeaPlotForces from "./components/fea-plot-forces.js";


customElements.define("fea-app", FeaApp);
customElements.define("fea-app-title-bar", FeaAppTitleBar);

customElements.define("fea-preprocessor", FeaPreprocessor);
customElements.define("fea-preprocessor-menu-buttons", FeaPreprocessorMenuButtons);

customElements.define("fea-geometry", FeaGeometry);

customElements.define("fea-geometry-menu", FeaGeometryMenu);
customElements.define("fea-geometry-menu-buttons", FeaGeometryMenuButtons);
customElements.define("fea-geometry-point-menu", FeaGeometryPointMenu);
customElements.define("fea-geometry-point-menu-buttons", FeaGeometryPointMenuButtons);
customElements.define("fea-geometry-line-menu", FeaGeometryLineMenu);
customElements.define("fea-geometry-line-menu-buttons", FeaGeometryLineMenuButtons);
customElements.define("fea-geometry-add-point-menu", FeaGeometryAddPointMenu);
customElements.define("fea-geometry-update-point-menu", FeaGeometryUpdatePointMenu);


customElements.define("fea-properties", FeaProperties);
customElements.define("fea-mesh", FeaMesh);
customElements.define("fea-analysis", FeaAnalysis);
customElements.define("fea-boundary-condition", FeaBoundaryCondition);
customElements.define("fea-load", FeaLoad);
customElements.define("fea-renderer", FeaRenderer);
customElements.define("fea-postprocessor", FeaPostprocessor);
customElements.define("fea-postprocessor-menu", FeaPostprocessorMenu);
customElements.define("fea-plot-displacements", FeaPlotDisplacements);
customElements.define("fea-plot-stresses", FeaPlotStresses);
customElements.define("fea-plot-strains", FeaPlotStrains);
customElements.define("fea-plot-forces", FeaPlotForces);
