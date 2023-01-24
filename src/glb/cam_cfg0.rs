#[doc = "Register `cam_cfg0` reader"]
pub struct R(crate::R<CAM_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAM_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAM_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAM_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cam_cfg0` writer"]
pub struct W(crate::W<CAM_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAM_CFG0_SPEC>;
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
impl From<crate::W<CAM_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAM_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_26` reader - "]
pub type RESERVED_0_26_R = crate::FieldReader<u32, u32>;
#[doc = "Field `reg_cam_ref_clk_en` reader - "]
pub type REG_CAM_REF_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_cam_ref_clk_en` writer - "]
pub type REG_CAM_REF_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAM_CFG0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:26"]
    #[inline(always)]
    pub fn reserved_0_26(&self) -> RESERVED_0_26_R {
        RESERVED_0_26_R::new(self.bits & 0x07ff_ffff)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_en(&self) -> REG_CAM_REF_CLK_EN_R {
        REG_CAM_REF_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cam_ref_clk_en(&mut self) -> REG_CAM_REF_CLK_EN_W<27> {
        REG_CAM_REF_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cam_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cam_cfg0](index.html) module"]
pub struct CAM_CFG0_SPEC;
impl crate::RegisterSpec for CAM_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cam_cfg0::R](R) reader structure"]
impl crate::Readable for CAM_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cam_cfg0::W](W) writer structure"]
impl crate::Writable for CAM_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cam_cfg0 to value 0"]
impl crate::Resettable for CAM_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
