#[doc = "Register `psram_configure` reader"]
pub struct R(crate::R<PSRAM_CONFIGURE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSRAM_CONFIGURE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSRAM_CONFIGURE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSRAM_CONFIGURE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psram_configure` writer"]
pub struct W(crate::W<PSRAM_CONFIGURE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSRAM_CONFIGURE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PSRAM_CONFIGURE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSRAM_CONFIGURE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_vendor_sel` reader - "]
pub type REG_VENDOR_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_vendor_sel` writer - "]
pub type REG_VENDOR_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_CONFIGURE_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `reg_ap_mr` reader - "]
pub type REG_AP_MR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_ap_mr` writer - "]
pub type REG_AP_MR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_CONFIGURE_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `reg_wb_reg_sel` reader - "]
pub type REG_WB_REG_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_wb_reg_sel` writer - "]
pub type REG_WB_REG_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_CONFIGURE_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_11` reader - "]
pub type RESERVED_11_R = crate::BitReader<bool>;
#[doc = "Field `reg_config_w_pusle` writer - "]
pub type REG_CONFIG_W_PUSLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_config_r_pusle` writer - "]
pub type REG_CONFIG_R_PUSLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `sts_config_w_done` reader - "]
pub type STS_CONFIG_W_DONE_R = crate::BitReader<bool>;
#[doc = "Field `sts_config_r_done` reader - "]
pub type STS_CONFIG_R_DONE_R = crate::BitReader<bool>;
#[doc = "Field `reg_config_req` reader - "]
pub type REG_CONFIG_REQ_R = crate::BitReader<bool>;
#[doc = "Field `reg_config_req` writer - "]
pub type REG_CONFIG_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_config_gnt` reader - "]
pub type REG_CONFIG_GNT_R = crate::BitReader<bool>;
#[doc = "Field `reg_x16_mode` reader - "]
pub type REG_X16_MODE_R = crate::BitReader<bool>;
#[doc = "Field `reg_x16_mode` writer - "]
pub type REG_X16_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_wb_hyper3` reader - "]
pub type REG_WB_HYPER3_R = crate::BitReader<bool>;
#[doc = "Field `reg_wb_hyper3` writer - "]
pub type REG_WB_HYPER3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reg_pck_s_div` reader - "]
pub type REG_PCK_S_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_pck_s_div` writer - "]
pub type REG_PCK_S_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_CONFIGURE_SPEC, u8, u8, 3, O>;
#[doc = "Field `reg_clkn_free` reader - "]
pub type REG_CLKN_FREE_R = crate::BitReader<bool>;
#[doc = "Field `reg_clkn_free` writer - "]
pub type REG_CLKN_FREE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSRAM_CONFIGURE_SPEC, bool, O>;
#[doc = "Field `reserved_24_27` reader - "]
pub type RESERVED_24_27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_linear_bnd_b` reader - "]
pub type REG_LINEAR_BND_B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_linear_bnd_b` writer - "]
pub type REG_LINEAR_BND_B_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_CONFIGURE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn reg_vendor_sel(&self) -> REG_VENDOR_SEL_R {
        REG_VENDOR_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn reg_ap_mr(&self) -> REG_AP_MR_R {
        REG_AP_MR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn reg_wb_reg_sel(&self) -> REG_WB_REG_SEL_R {
        REG_WB_REG_SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reserved_11(&self) -> RESERVED_11_R {
        RESERVED_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sts_config_w_done(&self) -> STS_CONFIG_W_DONE_R {
        STS_CONFIG_W_DONE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sts_config_r_done(&self) -> STS_CONFIG_R_DONE_R {
        STS_CONFIG_R_DONE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_config_req(&self) -> REG_CONFIG_REQ_R {
        REG_CONFIG_REQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_config_gnt(&self) -> REG_CONFIG_GNT_R {
        REG_CONFIG_GNT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_x16_mode(&self) -> REG_X16_MODE_R {
        REG_X16_MODE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_wb_hyper3(&self) -> REG_WB_HYPER3_R {
        REG_WB_HYPER3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn reg_pck_s_div(&self) -> REG_PCK_S_DIV_R {
        REG_PCK_S_DIV_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_clkn_free(&self) -> REG_CLKN_FREE_R {
        REG_CLKN_FREE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reserved_24_27(&self) -> RESERVED_24_27_R {
        RESERVED_24_27_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reg_linear_bnd_b(&self) -> REG_LINEAR_BND_B_R {
        REG_LINEAR_BND_B_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_vendor_sel(&mut self) -> REG_VENDOR_SEL_W<0> {
        REG_VENDOR_SEL_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ap_mr(&mut self) -> REG_AP_MR_W<4> {
        REG_AP_MR_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_reg_sel(&mut self) -> REG_WB_REG_SEL_W<8> {
        REG_WB_REG_SEL_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reg_config_w_pusle(&mut self) -> REG_CONFIG_W_PUSLE_W<12> {
        REG_CONFIG_W_PUSLE_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reg_config_r_pusle(&mut self) -> REG_CONFIG_R_PUSLE_W<13> {
        REG_CONFIG_R_PUSLE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_config_req(&mut self) -> REG_CONFIG_REQ_W<16> {
        REG_CONFIG_REQ_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn reg_x16_mode(&mut self) -> REG_X16_MODE_W<18> {
        REG_X16_MODE_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wb_hyper3(&mut self) -> REG_WB_HYPER3_W<19> {
        REG_WB_HYPER3_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pck_s_div(&mut self) -> REG_PCK_S_DIV_W<20> {
        REG_PCK_S_DIV_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_clkn_free(&mut self) -> REG_CLKN_FREE_W<23> {
        REG_CLKN_FREE_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_linear_bnd_b(&mut self) -> REG_LINEAR_BND_B_W<28> {
        REG_LINEAR_BND_B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "psram_configure\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psram_configure](index.html) module"]
pub struct PSRAM_CONFIGURE_SPEC;
impl crate::RegisterSpec for PSRAM_CONFIGURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psram_configure::R](R) reader structure"]
impl crate::Readable for PSRAM_CONFIGURE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psram_configure::W](W) writer structure"]
impl crate::Writable for PSRAM_CONFIGURE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psram_configure to value 0xa080_c002"]
impl crate::Resettable for PSRAM_CONFIGURE_SPEC {
    const RESET_VALUE: Self::Ux = 0xa080_c002;
}
