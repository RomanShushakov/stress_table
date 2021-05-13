import HidingContentButton from "./common_components/hiding-content-button.js";
customElements.define("hiding-content-button", HidingContentButton);

import FeaApp from "./components/fea-app.js";
import FeaAppTitleBar from "./components/fea-app-title-bar.js";
import FeaPreprocessor from "./components/fea-preprocessor.js";
// import FeaPreprocessorMenu from "./components/fea-preprocessor-menu.js";
import FeaGeometry from "./components/fea-geometry.js";
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
// customElements.define("fea-preprocessor-menu", FeaPreprocessorMenu);
customElements.define("fea-geometry", FeaGeometry);
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
