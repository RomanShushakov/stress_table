import FeaApp from "./components/fea-app.js";
import FeaAppTitleBar from "./components/fea-app-title-bar.js";

import FeaRenderer from "./components/fea-renderer.js";

import FeaPreprocessorMenu from "./components/fea-preprocessor-menu.js";
import FeaPreprocessorMenuButtons from "./components/fea-preprocessor-menu-buttons.js";

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


customElements.define("fea-app", FeaApp);
customElements.define("fea-app-title-bar", FeaAppTitleBar);

customElements.define("fea-renderer", FeaRenderer);

customElements.define("fea-preprocessor-menu", FeaPreprocessorMenu);
customElements.define("fea-preprocessor-menu-buttons", FeaPreprocessorMenuButtons);

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
